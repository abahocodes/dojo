//! Slash-command registry.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Practice,
    Session,
    Browse,
    Insight,
    Settings,
    App,
}

impl Group {
    pub const ORDER: &[Group] = &[
        Group::Practice,
        Group::Session,
        Group::Browse,
        Group::Insight,
        Group::Settings,
        Group::App,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Group::Practice => "Practice",
            Group::Session => "During a session",
            Group::Browse => "Browse",
            Group::Insight => "Insight",
            Group::Settings => "Settings",
            Group::App => "App",
        }
    }
}

pub struct Spec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub args: &'static str,
    pub about: &'static str,
    pub group: Group,
    /// Milestone that delivers the command, `None` when available now.
    pub soon: Option<&'static str>,
}

impl Spec {
    pub fn takes_args(&self) -> bool {
        !self.args.is_empty()
    }
}

const fn spec(
    name: &'static str,
    args: &'static str,
    about: &'static str,
    group: Group,
    soon: Option<&'static str>,
) -> Spec {
    Spec {
        name,
        aliases: &[],
        args,
        about,
        group,
        soon,
    }
}

pub const COMMANDS: &[Spec] = &[
    spec(
        "solve",
        "<id | query | need | random> [N]",
        "start fresh on one or more questions",
        Group::Practice,
        None,
    ),
    spec("test", "", "run the visible tests", Group::Session, None),
    spec(
        "submit",
        "",
        "run every test, including hidden ones",
        Group::Session,
        None,
    ),
    spec(
        "hint",
        "[id] [n]",
        "reveal the next hint",
        Group::Session,
        None,
    ),
    spec(
        "solution",
        "[id]",
        "show the explained solution",
        Group::Session,
        None,
    ),
    spec(
        "edit",
        "[id]",
        "back to your question: reopen the editor, or continue unfinished work",
        Group::Session,
        None,
    ),
    spec("pause", "", "pause the timer", Group::Session, None),
    spec(
        "skip",
        "",
        "move on without solving it",
        Group::Session,
        None,
    ),
    spec(
        "next",
        "",
        "move to the next question",
        Group::Session,
        None,
    ),
    spec(
        "list",
        "[query | tag | company]",
        "browse questions",
        Group::Browse,
        None,
    ),
    spec(
        "show",
        "<id | slug>",
        "read a problem statement",
        Group::Browse,
        None,
    ),
    spec(
        "past",
        "[id] [n]",
        "your past attempts and their code",
        Group::Browse,
        None,
    ),
    spec(
        "report",
        "[tag | company]",
        "mastery, gaps, history and what to practice next",
        Group::Insight,
        None,
    ),
    spec(
        "editor",
        "[name | command]",
        "show or set your editor",
        Group::Settings,
        None,
    ),
    spec(
        "lang",
        "[language]",
        "show or set your solve language",
        Group::Settings,
        None,
    ),
    spec(
        "config",
        "",
        "show settings and file locations",
        Group::Settings,
        None,
    ),
    spec(
        "contribute",
        "[new | key | claude | openai | ollama]",
        "add a question: describe it, review the draft, open a PR",
        Group::App,
        None,
    ),
    spec(
        "copy",
        "",
        "copy the last output to the clipboard",
        Group::App,
        None,
    ),
    spec(
        "accept",
        "",
        "open the pull request for your drafted question",
        Group::App,
        None,
    ),
    spec("donate", "", "support dojo's development", Group::App, None),
    spec("clear", "", "clear the screen", Group::App, None),
    spec("help", "[command]", "list commands", Group::App, None),
    Spec {
        name: "quit",
        aliases: &["exit", "q"],
        args: "",
        about: "close dojo: pauses any open question and prints your summary",
        group: Group::App,
        soon: None,
    },
];

/// Looks a command up by name or alias, ignoring case (`/Solve` works).
pub fn find(name: &str) -> Option<&'static Spec> {
    let name = name.to_ascii_lowercase();
    COMMANDS
        .iter()
        .find(|c| c.name == name || c.aliases.contains(&name.as_str()))
}

/// The command a mistyped name most likely meant (`/sovle` → `solve`):
/// the closest name or alias within two edits, for "did you mean".
pub fn closest(name: &str) -> Option<&'static str> {
    let name = name.to_ascii_lowercase();
    COMMANDS
        .iter()
        .filter(|c| c.soon.is_none())
        .flat_map(|c| {
            std::iter::once(c.name)
                .chain(c.aliases.iter().copied())
                .map(move |n| (c.name, n))
        })
        .map(|(command, n)| (edit_distance(&name, n), command))
        .filter(|(d, _)| *d <= 2.min(name.len().saturating_sub(1)))
        .min_by_key(|(d, _)| *d)
        .map(|(_, command)| command)
}

/// Edits (insert, delete, substitute, swap two neighbours) between two
/// words.
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    d[0] = (0..=b.len()).collect();
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[a.len()][b.len()]
}

/// How many arguments a command accepts, when that's a fixed number. Free
/// text (`/solve`, `/list`, `/show`, `/editor`) and `[id] [n]` forms,
/// which also accept search words, check their own.
pub fn max_args(spec: &Spec) -> Option<usize> {
    match spec.name {
        _ if !spec.takes_args() => Some(0),
        "lang" | "help" | "report" | "contribute" | "edit" => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::exact("solve", Some("solve"))]
    #[case::upper("SOLVE", Some("solve"))]
    #[case::mixed("Help", Some("help"))]
    #[case::alias("Q", Some("quit"))]
    #[case::unknown("sovle", None)]
    fn finds_commands_ignoring_case(#[case] name: &str, #[case] found: Option<&str>) {
        assert_eq!(find(name).map(|s| s.name), found);
    }

    #[rstest]
    #[case::swap("sovle", Some("solve"))]
    #[case::swap_list("lsit", Some("list"))]
    #[case::missing_letter("sumbit", Some("submit"))]
    #[case::extra_letter("helpp", Some("help"))]
    #[case::alias_typo("exti", Some("quit"))]
    #[case::case("SOVLE", Some("solve"))]
    #[case::nonsense("xyz", None)]
    #[case::too_short("x", None)]
    fn suggests_the_closest_command(#[case] name: &str, #[case] expected: Option<&str>) {
        assert_eq!(closest(name), expected);
    }
}
