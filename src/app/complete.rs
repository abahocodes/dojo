//! Slash-command autocomplete.

use super::commands::{COMMANDS, Spec, find};
use crate::config::EDITOR_PRESETS;
use crate::questions::Bank;
use crate::questions::search::{rank, search};
use crate::runner::Language;

pub const MAX_ITEMS: usize = 8;

#[derive(Debug, Clone)]
pub struct Item {
    /// Full input line after accepting.
    pub replacement: String,
    pub label: String,
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
}

/// Whether a command does anything useful in this context.
pub fn applies(name: &str, ctx: Context) -> bool {
    match name {
        "test" | "submit" | "pause" | "skip" => ctx.attempt,
        "next" => ctx.session && !ctx.attempt,
        "solve" | "random" | "need" => !ctx.attempt,
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
            Some(spec) if takes_args(spec, ctx) => arguments(bank, spec, line, args),
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
            Item {
                replacement: if args {
                    format!("/{} ", c.name)
                } else {
                    format!("/{}", c.name)
                },
                label: format!("/{}", c.name),
                detail: match c.soon {
                    Some(m) => format!("{} · {m}", c.about),
                    None => c.about.to_string(),
                },
                submit: !args,
            }
        })
        .collect()
}

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
            let mut values = bank.labels();
            values.extend(["easy", "medium", "hard"].map(String::from));
            if spec.name == "solve" && position == 0 {
                values.insert(0, "random".into());
            }
            rank(token, &values, |v| v.clone())
                .into_iter()
                .filter(|v| v.as_str() != token)
                .take(MAX_ITEMS)
                .map(|v| Item {
                    replacement: replace(v, false),
                    label: v.clone(),
                    detail: label_detail(bank, v),
                    submit: false,
                })
                .collect()
        }
        "editor" if position == 0 => {
            simple(rank(token, EDITOR_PRESETS, |s| s.to_string()), &replace)
        }
        "lang" if position == 0 => {
            let langs: Vec<&str> = Language::ALL.iter().map(|l| l.name()).collect();
            simple(rank(token, &langs, |s| s.to_string()), &replace)
        }
        "help" if position == 0 => {
            let names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
            simple(rank(token, &names, |s| s.to_string()), &replace)
        }
        _ => vec![],
    }
}

fn simple(values: Vec<&&str>, replace: &dyn Fn(&str, bool) -> String) -> Vec<Item> {
    values
        .into_iter()
        .take(MAX_ITEMS)
        .map(|v| Item {
            replacement: replace(v, true),
            label: v.to_string(),
            detail: String::new(),
            submit: true,
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
            Item {
                replacement,
                label: format!("{:>3}  {}", q.meta.id, q.meta.title),
                detail: format!("{} · {}", q.meta.difficulty, q.meta.tags.join(" ")),
                submit,
            }
        })
        .collect()
}

fn label_detail(bank: &Bank, label: &str) -> String {
    let n = bank
        .all()
        .iter()
        .filter(|q| {
            q.meta.tags.iter().any(|t| t == label)
                || q.meta.companies.iter().any(|c| c == label)
                || q.meta.difficulty.to_string() == label
        })
        .count();
    format!("{n} question{}", if n == 1 { "" } else { "s" })
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDLE: Context = Context {
        attempt: false,
        session: false,
    };
    const SOLVING: Context = Context {
        attempt: true,
        session: true,
    };

    #[test]
    fn suggestions_follow_context() {
        let bank = Bank::embedded();
        // Mid-question: /hint runs on Enter, no question ids offered.
        let items = complete(&bank, "/hi", SOLVING);
        assert_eq!(items[0].replacement, "/hint");
        assert!(items[0].submit);
        assert!(complete(&bank, "/hint ", SOLVING).is_empty());
        // Outside a session it needs a question.
        assert_eq!(complete(&bank, "/hi", IDLE)[0].replacement, "/hint ");
        assert!(!complete(&bank, "/hint ", IDLE).is_empty());
        // Irrelevant commands are hidden; relevant ones lead.
        let all: Vec<String> = complete(&bank, "/", SOLVING)
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert_eq!(all[0], "/test");
        assert!(!all.contains(&"/solve".to_string()));
        let idle: Vec<String> = complete(&bank, "/", IDLE)
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(!idle.contains(&"/test".to_string()));
        assert_eq!(idle[0], "/solve");
        // Removed and unbuilt commands are never suggested.
        for gone in ["/resume", "/end", "/giveup", "/history", "/random", "/past"] {
            assert!(!idle.contains(&gone.to_string()), "{gone} suggested");
            assert!(!all.contains(&gone.to_string()), "{gone} suggested");
        }
    }

    #[test]
    fn completes_command_names() {
        let bank = Bank::embedded();
        let items = complete(&bank, "/sh", IDLE);
        assert_eq!(items[0].label, "/show");
        assert_eq!(items[0].replacement, "/show ");
        assert!(!items[0].submit);
        let items = complete(&bank, "/cle", IDLE);
        assert_eq!(items[0].replacement, "/clear");
        assert!(items[0].submit);
    }

    #[test]
    fn completes_question_ids() {
        let bank = Bank::embedded();
        let items = complete(&bank, "/show 1", IDLE);
        assert!(items.iter().any(|i| i.replacement == "/show 1"));
        assert!(items.iter().all(|i| i.submit));
    }

    #[test]
    fn ignores_plain_text() {
        assert!(complete(&Bank::embedded(), "hello", IDLE).is_empty());
    }
}
