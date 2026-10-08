//! First-run setup: pick an editor and a language, then save the config.
//!
//! Runs when there's no config file yet. Every step has a default, so Enter
//! through it works; Esc keeps the defaults and finishes.

use ratatui::text::Span;

use super::complete::Item;
use super::transcript::Entry;
use super::{App, views};
use crate::config::{EDITOR_PRESETS, editor_preset};
use crate::runner::Language;
use crate::ui::text::Para;
use crate::ui::theme::theme;

/// One editor that can be picked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    /// What the user types or sees (`nvim`, `vscode`).
    pub name: String,
    /// The command saved to the config (`nvim {file}`).
    pub command: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Editor(Vec<Choice>),
    Language(Vec<(Language, Result<String, String>)>),
}

pub struct Setup {
    pub step: Step,
}

/// Editors to offer: `$VISUAL`/`$EDITOR` first, then presets found on the
/// PATH (aliases of the same program once), then `vi` as a last resort.
pub fn editor_choices(env_editor: Option<&str>, on_path: impl Fn(&str) -> bool) -> Vec<Choice> {
    let mut out: Vec<Choice> = Vec::new();
    if let Some(e) = env_editor.map(str::trim).filter(|e| !e.is_empty()) {
        out.push(Choice {
            name: e.split_whitespace().next().unwrap_or(e).to_string(),
            command: e.to_string(),
            note: "from $VISUAL/$EDITOR".into(),
        });
    }
    for name in EDITOR_PRESETS {
        let command = editor_preset(name).unwrap_or_default();
        let program = command.split_whitespace().next().unwrap_or_default();
        let taken = out
            .iter()
            .any(|c| c.command.split_whitespace().next() == Some(program));
        if !taken && on_path(program) {
            out.push(Choice {
                name: name.to_string(),
                command: command.to_string(),
                note: if is_terminal(program) {
                    "in the terminal".into()
                } else {
                    "opens its own window".into()
                },
            });
        }
    }
    if out.is_empty() {
        out.push(Choice {
            name: "vi".into(),
            command: "vi {file}".into(),
            note: "the only editor found".into(),
        });
    }
    out
}

fn is_terminal(program: &str) -> bool {
    matches!(
        program,
        "vim" | "nvim" | "vi" | "emacs" | "emacsclient" | "hx" | "nano" | "micro"
    )
}

/// What an answer picks: a number from the list, a listed name, or any
/// preset or command.
pub fn pick_editor(choices: &[Choice], answer: &str) -> Option<String> {
    let answer = answer.trim();
    if answer.is_empty() {
        return choices.first().map(|c| c.command.clone());
    }
    if let Ok(n) = answer.parse::<usize>() {
        return choices.get(n.checked_sub(1)?).map(|c| c.command.clone());
    }
    if let Some(c) = choices.iter().find(|c| c.name == answer) {
        return Some(c.command.clone());
    }
    Some(
        editor_preset(answer)
            .map(str::to_string)
            .unwrap_or_else(|| answer.to_string()),
    )
}

pub fn pick_language(
    choices: &[(Language, Result<String, String>)],
    answer: &str,
) -> Option<Language> {
    let answer = answer.trim();
    if answer.is_empty() {
        // The first one that's installed.
        return choices
            .iter()
            .find(|(_, v)| v.is_ok())
            .or(choices.first())
            .map(|(l, _)| *l);
    }
    if let Ok(n) = answer.parse::<usize>() {
        return choices.get(n.checked_sub(1)?).map(|(l, _)| *l);
    }
    Language::parse(answer)
}

fn numbered(i: usize, name: &str, note: &str, default: bool) -> Para {
    let t = theme();
    let mut spans = vec![
        Span::styled(format!("{}. ", i + 1), t.accent()),
        Span::styled(name.to_string(), t.bold()),
        Span::styled(format!("  {note}"), t.dim()),
    ];
    if default {
        spans.push(Span::styled("  ⏎ default", t.accent()));
    }
    Para::new(spans).indent(2)
}

fn editor_entry(choices: &[Choice]) -> Entry {
    let t = theme();
    let mut paras = vec![
        Para::plain("Let's set you up", t.heading()),
        Para::plain(
            "You'll solve questions in your own editor. Which one?",
            t.text(),
        ),
        Para::blank(),
    ];
    for (i, c) in choices.iter().enumerate() {
        paras.push(numbered(i, &c.name, &c.note, i == 0));
    }
    paras.push(Para::blank());
    paras.push(Para::plain(
        "Type a number or a name (any command works, e.g. `code --wait`).",
        t.dim(),
    ));
    views::entry(paras)
}

fn language_entry(choices: &[(Language, Result<String, String>)]) -> Entry {
    let t = theme();
    let default = pick_language(choices, "");
    let mut paras = vec![Para::plain("Which language do you practice in?", t.text())];
    paras.push(Para::blank());
    for (i, (lang, version)) in choices.iter().enumerate() {
        let note = match version {
            Ok(v) => v.clone(),
            Err(_) => "not installed".into(),
        };
        paras.push(numbered(i, lang.label(), &note, Some(*lang) == default));
    }
    paras.push(Para::blank());
    paras.push(Para::plain("You can switch any time with /lang.", t.dim()));
    views::entry(paras)
}

impl App {
    /// Starts setup on the first launch (no config file yet).
    pub(super) fn maybe_start_setup(&mut self) {
        if self.paths.config_file.exists() {
            return;
        }
        let env = std::env::var("VISUAL")
            .ok()
            .filter(|e| !e.trim().is_empty())
            .or_else(|| std::env::var("EDITOR").ok());
        let choices = editor_choices(env.as_deref(), super::which);
        self.transcript.push(editor_entry(&choices));
        self.setup = Some(Setup {
            step: Step::Editor(choices),
        });
    }

    /// Handles a line typed during setup. Returns false when setup isn't
    /// running (or the line is a command), so it runs as usual.
    pub(super) fn setup_text(&mut self, line: &str) -> bool {
        if line.starts_with('/') {
            return false;
        }
        let Some(setup) = &self.setup else {
            return false;
        };
        match &setup.step {
            Step::Editor(choices) => {
                let Some(command) = pick_editor(choices, line) else {
                    self.transcript.push(views::error(format!(
                        "pick 1–{}, or type an editor's name",
                        choices.len()
                    )));
                    return true;
                };
                self.config.editor = Some(command);
                self.ask_language();
            }
            Step::Language(choices) => {
                let Some(lang) = pick_language(choices, line) else {
                    self.transcript
                        .push(views::error("pick 1–2, or type python or javascript"));
                    return true;
                };
                self.config.language = lang.name().to_string();
                self.finish_setup();
            }
        }
        true
    }

    fn ask_language(&mut self) {
        let choices: Vec<_> = Language::ALL
            .iter()
            .map(|l| {
                (
                    *l,
                    crate::runner::toolchain(*l).map_err(|e| format!("{e:#}")),
                )
            })
            .collect();
        self.transcript.push(language_entry(&choices));
        self.setup = Some(Setup {
            step: Step::Language(choices),
        });
    }

    /// Saves what was picked (defaults for anything skipped).
    pub(super) fn finish_setup(&mut self) {
        let Some(setup) = self.setup.take() else {
            return;
        };
        if let Step::Editor(choices) = &setup.step
            && self.config.editor.is_none()
        {
            self.config.editor = choices.first().map(|c| c.command.clone());
        }
        if let Some(err) = self.save_config() {
            self.transcript.push(err);
            return;
        }
        let (editor, _) = self.config.editor();
        let editor = editor.replace(" {file}", "");
        let lang = self.config.lang();
        let mut msg = format!(
            "all set: {editor} · {}  ·  change them any time with /editor and /lang",
            lang.label()
        );
        if let Err(e) = crate::runner::toolchain(lang) {
            msg.push_str(&format!("\n{e:#}"));
            self.transcript.push(views::warn(msg));
        } else {
            self.transcript.push(views::ok(msg));
        }
    }

    /// Guidance for the empty prompt during setup.
    pub(super) fn setup_prompt(&self) -> Option<super::Prompt> {
        let setup = self.setup.as_ref()?;
        let tip = |k: &str, t: &str| (k.to_string(), t.to_string());
        let (enter, what) = match &setup.step {
            Step::Editor(c) => (
                c.first().map(|c| c.name.clone()).unwrap_or_default(),
                "number or name",
            ),
            Step::Language(c) => (
                pick_language(c, "")
                    .map(|l| l.label().to_string())
                    .unwrap_or_default(),
                "number or name",
            ),
        };
        Some(super::Prompt {
            enter: Some((String::new(), format!("use {enter}"))),
            tips: vec![tip("type", what), tip("Esc", "keep the defaults")],
        })
    }

    /// Suggestions while setting up: the listed choices.
    pub(super) fn setup_completions(&self, typed: &str) -> Option<Vec<Item>> {
        let setup = self.setup.as_ref()?;
        if typed.starts_with('/') || typed.is_empty() {
            return Some(vec![]);
        }
        let names: Vec<(String, String)> = match &setup.step {
            Step::Editor(c) => c.iter().map(|c| (c.name.clone(), c.note.clone())).collect(),
            Step::Language(c) => c
                .iter()
                .map(|(l, _)| (l.name().to_string(), l.label().to_string()))
                .collect(),
        };
        Some(
            names
                .into_iter()
                .filter(|(n, _)| n.starts_with(typed) && n != typed)
                .map(|(n, d)| {
                    Item::builder()
                        .replacement(n.clone())
                        .label(n)
                        .detail(d)
                        .submit(true)
                        .build()
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn found(programs: &'static [&'static str]) -> impl Fn(&str) -> bool {
        move |p| programs.contains(&p)
    }

    fn names(c: &[Choice]) -> Vec<&str> {
        c.iter().map(|c| c.name.as_str()).collect()
    }

    #[rstest]
    #[case::env_first(Some("nvim"), &["code", "nvim"], &["nvim", "vscode"])]
    #[case::env_with_args(Some("code --wait"), &["code", "vim"], &["code", "vim"])]
    #[case::aliases_once(None, &["code", "nvim", "emacs", "emacsclient"], &["vscode", "nvim", "emacs", "emacsclient"])]
    #[case::nothing_found(None, &[], &["vi"])]
    #[case::blank_env_ignored(Some("  "), &["nano"], &["nano"])]
    fn offers_editors(
        #[case] env: Option<&str>,
        #[case] on_path: &'static [&'static str],
        #[case] expected: &[&str],
    ) {
        assert_eq!(names(&editor_choices(env, found(on_path))), expected);
    }

    #[rstest]
    #[case::enter_takes_first("", Some("code {file}"))]
    #[case::number("2", Some("nvim {file}"))]
    #[case::listed_name("vscode", Some("code {file}"))]
    #[case::other_preset("helix", Some("hx {file}"))]
    #[case::any_command("subl -w", Some("subl -w"))]
    #[case::out_of_range("9", None)]
    #[case::zero("0", None)]
    fn picks_editors(#[case] answer: &str, #[case] expected: Option<&str>) {
        let choices = editor_choices(None, found(&["nvim", "code"]));
        assert_eq!(pick_editor(&choices, answer).as_deref(), expected);
    }

    #[rstest]
    #[case::enter_takes_installed("", Some(Language::JavaScript))]
    #[case::number("1", Some(Language::Python))]
    #[case::name("javascript", Some(Language::JavaScript))]
    #[case::alias("js", Some(Language::JavaScript))]
    #[case::unknown("cobol", None)]
    fn picks_languages(#[case] answer: &str, #[case] expected: Option<Language>) {
        let choices = vec![
            (Language::Python, Err("python3 not found".to_string())),
            (Language::JavaScript, Ok("Node.js v22".to_string())),
        ];
        assert_eq!(pick_language(&choices, answer), expected);
    }
}
