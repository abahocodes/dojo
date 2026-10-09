//! Slash-command autocomplete.

use super::commands::{COMMANDS, Spec, find};
use crate::config::EDITOR_PRESETS;
use crate::questions::Bank;
use crate::questions::search::{rank, search};
use crate::runner::Language;

pub const MAX_ITEMS: usize = 8;

#[derive(Debug, Clone, bon::Builder)]
pub struct Item {
    /// Full input line after accepting.
    pub replacement: String,
    pub label: String,
    #[builder(default)]
    pub detail: String,
    /// Accepting with Enter also submits the line.
    pub submit: bool,
}

/// Where the user is, so suggestions only offer what makes sense now.
#[derive(Debug, Clone, Copy, Default)]
pub struct Context {
    /// A question is open.
    pub attempt: bool,
    /// A session is running (possibly between questions).
    pub session: bool,
    /// A drafted question is being reviewed (`/contribute`).
    pub review: bool,
}

/// Whether a command does anything useful in this context.
pub fn applies(name: &str, ctx: Context) -> bool {
    match name {
        "test" | "submit" | "pause" | "skip" => ctx.attempt,
        "next" => ctx.session && !ctx.attempt,
        "solve" | "random" | "need" => !ctx.attempt,
        "accept" => ctx.review,
        _ => true,
    }
}

/// In a session, `/hint` and `/solution` act on the current question and
/// take no argument.
fn takes_args(c: &Spec, ctx: Context) -> bool {
    match c.name {
        "hint" | "solution" if ctx.session => false,
        // `/edit` alone does the right thing; an id is optional.
        "edit" => false,
        _ => c.takes_args(),
    }
}

/// How early a command shows when the list is unfiltered: what you'd most
/// likely do next comes first.
fn priority(name: &str, ctx: Context) -> usize {
    let order: &[&str] = if ctx.attempt {
        &[
            "test", "submit", "hint", "edit", "solution", "skip", "pause",
        ]
    } else if ctx.session {
        &["next", "solution", "solve", "quit"]
    } else {
        &["solve", "list", "show"]
    };
    order.iter().position(|n| *n == name).unwrap_or(order.len())
}

pub fn complete(bank: &Bank, line: &str, ctx: Context) -> Vec<Item> {
    let Some(rest) = line.strip_prefix('/') else {
        return vec![];
    };
    match rest.split_once(' ') {
        None => commands(rest, ctx),
        Some((name, args)) => match find(name) {
            Some(spec) if takes_args(spec, ctx) => arguments()
                .bank(bank)
                .spec(spec)
                .line(line)
                .args(args)
                .call(),
            _ => vec![],
        },
    }
}

fn commands(prefix: &str, ctx: Context) -> Vec<Item> {
    // Unbuilt commands stay out of the list (`/help` still mentions them).
    let candidates: Vec<&Spec> = COMMANDS
        .iter()
        .filter(|c| c.soon.is_none() && applies(c.name, ctx))
        .collect();
    // Prefix matches first (most relevant first), then fuzzy matches.
    let mut out: Vec<&Spec> = candidates
        .iter()
        .copied()
        .filter(|c| c.name.starts_with(prefix) || c.aliases.iter().any(|a| a.starts_with(prefix)))
        .collect();
    out.sort_by_key(|c| priority(c.name, ctx));
    if prefix.len() >= 2 {
        for c in rank(prefix, &candidates, |c| c.name.to_string()) {
            if !out.iter().any(|o| o.name == c.name) {
                out.push(c);
            }
        }
    }
    out.into_iter()
        .take(MAX_ITEMS)
        .map(|c| {
            let args = takes_args(c, ctx);
            Item::builder()
                .replacement(if args {
                    format!("/{} ", c.name)
                } else {
                    format!("/{}", c.name)
                })
                .label(format!("/{}", c.name))
                .detail(match c.soon {
                    Some(m) => format!("{} · {m}", c.about),
                    None => c.about.to_string(),
                })
                .submit(!args)
                .build()
        })
        .collect()
}

#[bon::builder]
fn arguments(bank: &Bank, spec: &Spec, line: &str, args: &str) -> Vec<Item> {
    // Complete the token under the (end-of-line) cursor.
    let (head, token) = match line.rfind(' ') {
        Some(i) => (&line[..=i], &line[i + 1..]),
        None => return vec![],
    };
    let position = args.split(' ').count() - 1; // index of the current token
    let previous = args.split(' ').rev().nth(1).unwrap_or("");
    if token.starts_with('-') || previous.starts_with('-') {
        return vec![];
    }

    let replace =
        |value: &str, submit: bool| format!("{head}{value}{}", if submit { "" } else { " " });

    match spec.name {
        "show" | "past" | "hint" | "solution" if position == 0 => {
            questions(bank, token, |id| (replace(&id.to_string(), true), true))
        }
        "solve"
            if position == 0 && token.chars().all(|c| c.is_ascii_digit()) && !token.is_empty() =>
        {
            questions(bank, token, |id| (replace(&id.to_string(), true), true))
        }
        "solve" | "list" | "report" => {
            let values = choices(bank, spec.name, position);
            rank(token, &values, |v| v.clone())
                .into_iter()
                .filter(|v| v.as_str() != token)
                .take(MAX_ITEMS)
                .map(|v| {
                    Item::builder()
                        .replacement(replace(v, false))
                        .label(v.clone())
                        .detail(label_detail(bank, v))
                        .submit(false)
                        .build()
                })
                .collect()
        }
        "editor" | "lang" | "help" if position == 0 => {
            let values = choices(bank, spec.name, position);
            simple(rank(token, &values, |s| s.clone()), &replace)
        }
        _ => vec![],
    }
}

/// The named values an argument can take (not question ids).
fn choices(bank: &Bank, command: &str, position: usize) -> Vec<String> {
    match command {
        "solve" | "list" | "report" => {
            let mut values = bank.labels();
            values.extend(["easy", "medium", "hard"].map(String::from));
            if command == "solve" && position == 0 {
                values.insert(0, "need".into());
                values.insert(1, "random".into());
            }
            values
        }
        "editor" if position == 0 => EDITOR_PRESETS.iter().map(|s| s.to_string()).collect(),
        "lang" if position == 0 => Language::ALL.iter().map(|l| l.name().to_string()).collect(),
        "help" if position == 0 => COMMANDS.iter().map(|c| c.name.to_string()).collect(),
        _ => vec![],
    }
}

/// Whether Enter should take the selected suggestion instead of running the
/// line as typed. Only a partly typed command name, or a partly typed value
/// from a fixed set (`/lang py`, `/help sol`, `/report gra`), completes.
/// Everything else runs as typed, Tab being the way to take a suggestion:
/// free text (`/solve two sum`, `/list dp`, `/show two`), editor commands
/// (`/editor subl`), ids, counts and exact values. `/solve need` means
/// `need`, not a fuzzy match like `indeed`.
pub fn enter_completes(bank: &Bank, line: &str, ctx: Context) -> bool {
    let Some(rest) = line.strip_prefix('/') else {
        return false;
    };
    let Some((name, args)) = rest.split_once(' ') else {
        return true;
    };
    let Some(spec) = find(name) else {
        return false;
    };
    if !takes_args(spec, ctx) {
        return false;
    }
    let token = args.rsplit_once(' ').map_or(args, |(_, t)| t);
    let position = args.split(' ').count() - 1;
    let fixed_set = matches!((spec.name, position), ("lang" | "help" | "report", 0));
    fixed_set
        && !token.is_empty()
        && !choices(bank, spec.name, position)
            .iter()
            .any(|v| v == token)
}

fn simple(values: Vec<&String>, replace: &dyn Fn(&str, bool) -> String) -> Vec<Item> {
    values
        .into_iter()
        .take(MAX_ITEMS)
        .map(|v| {
            Item::builder()
                .replacement(replace(v, true))
                .label(v.to_string())
                .submit(true)
                .build()
        })
        .collect()
}

fn questions(bank: &Bank, token: &str, make: impl Fn(u32) -> (String, bool)) -> Vec<Item> {
    let matches: Vec<_> = if token.chars().all(|c| c.is_ascii_digit()) {
        bank.all()
            .iter()
            .filter(|q| q.meta.id.to_string().starts_with(token))
            .collect()
    } else {
        search(bank, token)
    };
    matches
        .into_iter()
        .take(MAX_ITEMS)
        .map(|q| {
            let (replacement, submit) = make(q.meta.id);
            Item::builder()
                .replacement(replacement)
                .label(format!("{:>3}  {}", q.meta.id, q.meta.title))
                .detail(format!("{} · {}", q.meta.difficulty, q.meta.tags.join(" ")))
                .submit(submit)
                .build()
        })
        .collect()
}

fn label_detail(bank: &Bank, label: &str) -> String {
    let n = bank
        .all()
        .iter()
        .filter(|q| {
            q.meta.tags.iter().any(|t| t == label)
                || q.meta.companies.contains(label)
                || q.meta.difficulty.to_string() == label
        })
        .count();
    format!("{n} question{}", if n == 1 { "" } else { "s" })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const IDLE: Context = Context {
        attempt: false,
        session: false,
        review: false,
    };
    const SOLVING: Context = Context {
        attempt: true,
        session: true,
        review: false,
    };
    const BETWEEN: Context = Context {
        attempt: false,
        session: true,
        review: false,
    };
    const REVIEWING: Context = Context {
        attempt: false,
        session: false,
        review: true,
    };

    fn labels(line: &str, ctx: Context) -> Vec<String> {
        complete(&Bank::embedded(), line, ctx)
            .into_iter()
            .map(|i| i.label)
            .collect()
    }

    #[rstest]
    #[case::show_takes_args("/sh", IDLE, "/show ", false)]
    #[case::clear_runs("/cle", IDLE, "/clear", true)]
    #[case::hint_needs_question_idle("/hi", IDLE, "/hint ", false)]
    #[case::hint_runs_mid_question("/hi", SOLVING, "/hint", true)]
    #[case::solution_runs_mid_question("/sol", SOLVING, "/solution", true)]
    #[case::edit_runs("/ed", IDLE, "/edit", true)]
    #[case::test_mid_question("/te", SOLVING, "/test", true)]
    #[case::next_between("/ne", BETWEEN, "/next", true)]
    #[case::accept_when_reviewing("/acc", REVIEWING, "/accept", true)]
    #[case::alias("/exi", IDLE, "/quit", true)]
    fn completes_commands(
        #[case] line: &str,
        #[case] ctx: Context,
        #[case] replacement: &str,
        #[case] submit: bool,
    ) {
        let items = complete(&Bank::embedded(), line, ctx);
        assert_eq!(items[0].replacement, replacement);
        assert_eq!(items[0].submit, submit);
    }

    #[rstest]
    #[case::first_mid_question(SOLVING, "/test")]
    #[case::first_idle(IDLE, "/solve")]
    #[case::first_between(BETWEEN, "/next")]
    fn most_relevant_first(#[case] ctx: Context, #[case] first: &str) {
        assert_eq!(labels("/", ctx)[0], first);
    }

    #[rstest]
    #[case(SOLVING, "/solve")]
    #[case(SOLVING, "/next")]
    #[case(IDLE, "/test")]
    #[case(IDLE, "/submit")]
    #[case(IDLE, "/skip")]
    #[case(IDLE, "/accept")]
    #[case(BETWEEN, "/test")]
    fn hides_what_does_not_apply(#[case] ctx: Context, #[case] hidden: &str) {
        assert!(!labels("/", ctx).contains(&hidden.to_string()));
    }

    #[rstest]
    fn never_suggests_removed_commands(
        #[values(IDLE, SOLVING, BETWEEN, REVIEWING)] ctx: Context,
        #[values("/resume", "/end", "/giveup", "/history", "/random", "/reset")] gone: &str,
    ) {
        assert!(!labels("/", ctx).contains(&gone.to_string()));
    }

    #[rstest]
    #[case::hint_mid_question("/hint ", SOLVING, true)]
    #[case::solution_mid_question("/solution ", SOLVING, true)]
    #[case::plain_text("hello", IDLE, true)]
    #[case::unknown_command("/nope ", IDLE, true)]
    #[case::show_lists_questions("/show ", IDLE, false)]
    #[case::hint_idle_lists_questions("/hint ", IDLE, false)]
    fn argument_suggestions(#[case] line: &str, #[case] ctx: Context, #[case] empty: bool) {
        assert_eq!(complete(&Bank::embedded(), line, ctx).is_empty(), empty);
    }

    #[rstest]
    #[case("/show 1", "/show 1")]
    #[case("/show 13", "/show 13")]
    #[case("/past 6", "/past 6")]
    fn completes_question_ids(#[case] line: &str, #[case] expected: &str) {
        let items = complete(&Bank::embedded(), line, IDLE);
        assert!(items.iter().any(|i| i.replacement == expected));
        assert!(items.iter().all(|i| i.submit));
    }

    #[rstest]
    #[case("/solve ", "need")]
    #[case("/solve ", "random")]
    #[case("/list gra", "graphs")]
    #[case("/editor nv", "nvim")]
    #[case("/lang ja", "javascript")]
    fn suggests_arguments(#[case] line: &str, #[case] label: &str) {
        assert!(
            labels(line, IDLE).contains(&label.to_string()),
            "{:?}",
            labels(line, IDLE)
        );
    }

    /// Enter runs the line as typed unless it's finishing a partial command
    /// name or fixed-set value (`/solve need` used to become `/solve
    /// indeed`, `/solve two sum` became `/solve two prefix-sum`).
    #[rstest]
    #[case::command_name("/sol", true)]
    #[case::partial_language("/lang ja", true)]
    #[case::partial_help("/help sol", true)]
    #[case::partial_report_label("/report gra", true)]
    #[case::keyword("/solve need", false)]
    #[case::keyword_then_space("/solve need ", false)]
    #[case::count("/solve need 3", false)]
    #[case::random_count("/solve random 3", false)]
    #[case::query("/solve two sum", false)]
    #[case::short_query("/solve dp", false)]
    #[case::partial_word_query("/solve nee", false)]
    #[case::ids("/solve 42 43", false)]
    #[case::list_query("/list two sum", false)]
    #[case::list_partial("/list grap", false)]
    #[case::show_search("/show two", false)]
    #[case::show_slug("/show two-sum", false)]
    #[case::show_id("/show 42", false)]
    #[case::past_count("/past 6 3", false)]
    #[case::hint_id("/hint 3", false)]
    #[case::editor_command("/editor subl", false)]
    #[case::editor_vi("/editor vi", false)]
    #[case::editor_flags("/editor code --wait", false)]
    #[case::exact_language("/lang go", false)]
    #[case::exact_report_label("/report graphs", false)]
    #[case::exact_help("/help solve", false)]
    #[case::plain_text("hello", false)]
    fn enter_completes_only_partial_names(#[case] line: &str, #[case] completes: bool) {
        assert_eq!(enter_completes(&Bank::embedded(), line, IDLE), completes);
    }
}
