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
        "<id | query | random> [-n N]",
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
        "<id>",
        "your past attempts and code",
        Group::Browse,
        Some("M3"),
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
        "",
        "author a new question and open a PR",
        Group::App,
        Some("M6"),
    ),
    spec(
        "copy",
        "",
        "copy the last output to the clipboard",
        Group::App,
        None,
    ),
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

pub fn find(name: &str) -> Option<&'static Spec> {
    COMMANDS
        .iter()
        .find(|c| c.name == name || c.aliases.contains(&name))
}
