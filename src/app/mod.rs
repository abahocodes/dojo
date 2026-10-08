//! The fullscreen app: state, event loop and command dispatch.

mod commands;
mod complete;
mod contribute;
mod input;
mod practice;
mod setup;
mod transcript;
mod views;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEventKind,
};

use crate::config::{Config, Paths, editor_preset};
use crate::questions::search::search;
use crate::questions::{Bank, Question};
use crate::runner::{Language, RunReport, Which};
use crate::session::{Done, Outcome, Session};
use crate::store::Store;
use crate::ui;
use crate::ui::report::{ReportView, Tab};

/// Builds the report from everything recorded so far.
pub fn build_report(store: &Store, bank: &Bank) -> Result<crate::model::Report> {
    let attempts: Vec<crate::model::Attempt> = store
        .attempts()?
        .iter()
        .filter_map(crate::model::Attempt::from_record)
        .collect();
    Ok(crate::model::build()
        .bank(bank)
        .attempts(&attempts)
        .now(jiff::Timestamp::now())
        .tz(&jiff::tz::TimeZone::system())
        .call())
}

/// `dojo report --json`: the report as JSON on stdout.
pub fn report_json() -> Result<String> {
    let paths = Paths::detect()?;
    let store = Store::open(&paths.db)?;
    let report = build_report(&store, &Bank::embedded())?;
    Ok(serde_json::to_string_pretty(&report)?)
}

pub use complete::{Item, MAX_ITEMS};
pub use transcript::{Entry, Transcript};
pub use views::ago as views_ago;

const HISTORY_LIMIT: usize = 500;

/// Guidance for the empty prompt.
pub struct Prompt {
    /// The command Enter runs, and its label.
    pub enter: Option<(String, String)>,
    /// Other things that make sense now: (key, description).
    pub tips: Vec<(String, String)>,
}

/// What to do after a command that took over the terminal finishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterCommand {
    Editor,
    GhInstall,
    GhLogin,
    OllamaInstall,
    OllamaUpdate,
    OllamaPull,
}

/// Messages from background work.
pub enum Msg {
    Contrib(contribute::ContribMsg),
    Tests {
        attempt_id: i64,
        /// Matches `App::running`; results of cancelled runs are dropped.
        run: u64,
        result: Result<RunReport, String>,
    },
}

pub struct App {
    pub bank: Bank,
    pub config: Config,
    pub paths: Paths,
    store: Store,
    pub transcript: Transcript,
    pub input: input::Input,
    pub completions: Vec<Item>,
    pub selected: usize,
    /// The user moved through the suggestions, so Enter takes the selected
    /// one even when the line already reads as finished.
    chose: bool,
    /// Esc hides suggestions until the input changes.
    dismissed: bool,
    pub notice: Option<(String, Instant)>,
    quit_armed: Option<Instant>,
    /// Transcript viewport height from the last draw, for paging.
    pub body_height: usize,
    /// Total transcript lines from the last draw, for clamping scroll.
    pub body_lines: usize,
    pub session: Option<Session>,
    /// Tests in flight and when they started.
    pub running: Option<(Which, Instant)>,
    /// Sequence number of the run in flight, and its cancel flag.
    run_seq: u64,
    cancel: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
    /// A command waiting for the main loop to hand it the terminal (a
    /// terminal editor, installing gh, signing in to GitHub).
    pending_command: Option<(String, AfterCommand)>,
    /// `/contribute` while active.
    pub contrib: Option<contribute::Contrib>,
    /// Masked input for an API key, for this provider.
    pub secret_for: Option<crate::contribute::llm::Provider>,
    job_seq: u64,
    /// When the open attempt's clock was last saved.
    last_autosave: Instant,
    /// Last key press, paste, scroll or save, for pausing when idle.
    last_activity: Instant,
    /// Set when the timer paused itself because the user stepped away: the
    /// moment they were last seen. The next activity resumes it.
    away: Option<Instant>,
    /// Unfinished attempts, most recent first (refreshed when sessions end).
    pub unfinished: Vec<crate::store::OpenAttempt>,
    /// The fullscreen report, while open.
    pub report: Option<ReportView>,
    /// First-run setup, while it runs.
    setup: Option<setup::Setup>,
    /// After `/quit`: unsolved questions whose explained solutions are still
    /// to be shown before leaving.
    /// (questions left, how many there were)
    review: Option<(Vec<u32>, usize)>,
    /// Questions whose explained solution was shown since dojo opened.
    explained: Vec<u32>,
    /// Results of the last session, printed to the terminal on exit.
    last_summary: Option<Vec<Done>>,
    quit: bool,
}

impl App {
    pub fn new() -> Result<App> {
        let paths = Paths::detect()?;
        let config = Config::load(&paths.config_file)?;
        let store = Store::open(&paths.db)?;
        let history = store.history(HISTORY_LIMIT)?;
        let bank = Bank::embedded();
        let unfinished = store.open_attempts().unwrap_or_default();
        let mut transcript = Transcript::default();
        transcript.push(views::welcome(
            &bank,
            &config,
            paths.config_file.exists(),
            &unfinished,
        ));
        let (tx, rx) = channel();
        let mut app = App {
            bank,
            config,
            paths,
            store,
            transcript,
            input: input::Input::with_history(history),
            completions: vec![],
            selected: 0,
            chose: false,
            dismissed: false,
            notice: None,
            quit_armed: None,
            body_height: 0,
            body_lines: 0,
            session: None,
            running: None,
            run_seq: 0,
            cancel: None,
            tx,
            rx,
            pending_command: None,
            contrib: None,
            secret_for: None,
            job_seq: 0,
            last_autosave: Instant::now(),
            last_activity: Instant::now(),
            away: None,
            report: None,
            setup: None,
            review: None,
            explained: Vec::new(),
            unfinished,
            last_summary: None,
            quit: false,
        };
        app.maybe_start_setup();
        Ok(app)
    }

    /// What Enter on an empty prompt does, and how to describe it.
    pub fn default_action(&self) -> Option<(String, String)> {
        self.prompt().enter
    }

    /// The empty prompt's guidance: what Enter does and what else makes sense
    /// right now.
    pub fn prompt(&self) -> Prompt {
        let tip = |key: &str, text: &str| (key.to_string(), text.to_string());
        let enter = |cmd: &str, label: String| Some((cmd.to_string(), label));

        if let Some(p) = self.setup_prompt() {
            return p;
        }
        if let Some(tips) = self.contrib_prompt() {
            return Prompt { enter: None, tips };
        }

        if self.running.is_some() {
            return Prompt {
                enter: None,
                tips: vec![tip("", "running tests…"), tip("Ctrl+C", "cancels")],
            };
        }
        let Some(s) = &self.session else {
            if let Some((review, total)) = &self.review {
                let total = *total;
                return match review.first().and_then(|id| self.bank.get(*id)) {
                    Some(q) => Prompt {
                        enter: enter(
                            &format!("/solution {}", q.meta.id),
                            format!(
                                "explained: #{} {} ({} of {total})",
                                q.meta.id,
                                q.meta.title,
                                total - review.len() + 1
                            ),
                        ),
                        tips: vec![tip("/quit", "leave now")],
                    },
                    None => Prompt {
                        enter: enter("/quit", "all reviewed: close dojo".into()),
                        tips: vec![tip("/solve <id>", "keep practicing")],
                    },
                };
            }
            if let Some(o) = self.unfinished.first()
                && let Some(q) = self.bank.get(o.question_id as u32)
            {
                return Prompt {
                    enter: enter(
                        "/edit",
                        format!("continue #{} {}", o.question_id, q.meta.title),
                    ),
                    tips: vec![tip("/solve <id>", "something else"), tip("/list", "browse")],
                };
            }
            return Prompt {
                enter: None,
                tips: vec![
                    tip("/solve 1", "to start"),
                    tip("/list graphs", "to browse"),
                    tip("/help", "everything"),
                ],
            };
        };

        if let Some(a) = &s.attempt {
            let hints_left = self
                .bank
                .get(a.question_id)
                .map_or(0, |q| q.hints.len().saturating_sub(a.hints_used));
            let hint = if hints_left > 0 {
                tip("/hint", &format!("a nudge ({hints_left} left)"))
            } else {
                tip("/solution", "see the answer")
            };
            if a.timer.paused() {
                return Prompt {
                    enter: enter("/edit", "back to work".into()),
                    tips: vec![tip("/quit", "stop for now")],
                };
            }
            return match a.last_run {
                Some((passed, total)) if passed == total && total > 0 => Prompt {
                    enter: enter("/submit", "submit: run the hidden tests too".into()),
                    tips: vec![tip("/edit", "keep editing"), hint],
                },
                None if a.test_runs == 0 => Prompt {
                    enter: enter("/edit", "back to the editor".into()),
                    tips: vec![tip("/test", "run tests"), hint],
                },
                _ => Prompt {
                    enter: enter("/edit", "back to the editor".into()),
                    tips: vec![tip("/test", "re-run"), hint, tip("/skip", "move on")],
                },
            };
        }

        // Between questions.
        let review = s
            .done
            .last()
            .filter(|d| d.outcome != Outcome::Unfinished)
            .map(|_| tip("/solution", "review the explained solution"));
        match s.queue.get(s.index + 1).and_then(|id| self.bank.get(*id)) {
            Some(q) => Prompt {
                enter: enter("/next", format!("next: #{} {}", q.meta.id, q.meta.title)),
                tips: review
                    .into_iter()
                    .chain([tip("/quit", "summary & exit")])
                    .collect(),
            },
            None => Prompt {
                enter: None,
                tips: review
                    .into_iter()
                    .chain([
                        tip("/solve <id>", "keep going"),
                        tip("/quit", "summary & exit"),
                    ])
                    .collect(),
            },
        }
    }

    pub fn popup_open(&self) -> bool {
        !self.completions.is_empty() && !self.dismissed
    }

    fn notify(&mut self, msg: impl Into<String>) {
        self.notice = Some((msg.into(), Instant::now()));
    }

    fn completion_context(&self) -> complete::Context {
        complete::Context {
            attempt: self.session.as_ref().is_some_and(|s| s.attempt.is_some()),
            session: self.session.is_some(),
            review: self.contrib_reviewing(),
        }
    }

    fn refresh_completions(&mut self) {
        let ctx = self.completion_context();
        self.completions = self
            .setup_completions(self.input.text())
            .unwrap_or_else(|| complete::complete(&self.bank, self.input.text(), ctx));
        self.selected = 0;
        self.chose = false;
        self.dismissed = false;
    }

    fn edited(&mut self) {
        self.refresh_completions();
    }

    fn max_scroll(&self) -> usize {
        self.body_lines.saturating_sub(self.body_height)
    }

    fn accept_completion(&mut self) -> Option<bool> {
        let item = self.completions.get(self.selected)?.clone();
        self.input.set(item.replacement);
        self.refresh_completions();
        Some(item.submit)
    }

    fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }
        if self.report.is_some() {
            self.on_report_key(key);
            return;
        }
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);
        if !(ctrl && key.code == KeyCode::Char('c')) {
            self.quit_armed = None;
        }

        match key.code {
            // Ctrl+C interrupts: a test run, then the input, then the
            // suggestions. On an empty prompt, twice quits (pausing any
            // attempt so it continues next time).
            KeyCode::Char('c') if ctrl => {
                if self.running.is_some() {
                    self.cancel_tests();
                } else if self.secret_for.take().is_some() {
                    self.input.set("");
                    self.transcript.push(views::info("key entry cancelled"));
                } else if self.cancel_contrib() {
                } else if !self.input.text().is_empty() {
                    self.input.set("");
                    self.edited();
                } else if self.popup_open() {
                    self.dismissed = true;
                } else if self
                    .quit_armed
                    .is_some_and(|t| t.elapsed() < Duration::from_secs(2))
                {
                    self.end_session(false);
                    self.quit = true;
                } else {
                    self.quit_armed = Some(Instant::now());
                    let attempt = self.session.as_ref().and_then(|s| s.attempt.as_ref());
                    self.notify(match attempt {
                        Some(a) => format!(
                            "Ctrl+C again to quit · #{} will be paused and continue next time",
                            a.question_id
                        ),
                        None => "Ctrl+C again to quit".into(),
                    });
                }
            }
            // Ctrl+D on an empty prompt quits, like a shell or REPL.
            KeyCode::Char('d') if ctrl => {
                if self.input.text().is_empty() {
                    self.end_session(false);
                    self.quit = true;
                }
            }
            KeyCode::Char('l') if ctrl => self.transcript.clear(),
            KeyCode::Char('a') if ctrl => self.input.home(),
            KeyCode::Char('e') if ctrl => self.input.end(),
            KeyCode::Char('u') if ctrl => {
                self.input.kill_to_start();
                self.edited();
            }
            KeyCode::Char('k') if ctrl => {
                self.input.kill_to_end();
                self.edited();
            }
            KeyCode::Char('w') if ctrl => {
                self.input.delete_word();
                self.edited();
            }
            KeyCode::Char(c) if !ctrl => {
                self.input.insert(c.encode_utf8(&mut [0; 4]));
                self.edited();
            }
            KeyCode::Backspace => {
                self.input.backspace();
                self.edited();
            }
            KeyCode::Delete => {
                self.input.delete();
                self.edited();
            }
            KeyCode::Left => self.input.left(),
            KeyCode::Right => self.input.right(),
            KeyCode::Home => self.input.home(),
            KeyCode::End => self.input.end(),
            KeyCode::Up if shift => self.transcript.scroll_by(1, self.max_scroll()),
            KeyCode::Down if shift => self.transcript.scroll_by(-1, self.max_scroll()),
            KeyCode::Up if self.popup_open() => {
                let n = self.completions.len().min(complete::MAX_ITEMS);
                self.selected = (self.selected + n - 1) % n;
                self.chose = true;
            }
            KeyCode::Down if self.popup_open() => {
                let n = self.completions.len().min(complete::MAX_ITEMS);
                self.selected = (self.selected + 1) % n;
                self.chose = true;
            }
            KeyCode::Up => {
                self.input.history_prev();
                self.completions.clear();
            }
            KeyCode::Down => {
                self.input.history_next();
                self.completions.clear();
            }
            KeyCode::PageUp => {
                let page = self.body_height.saturating_sub(2).max(1) as isize;
                self.transcript.scroll_by(page, self.max_scroll());
            }
            KeyCode::PageDown => {
                let page = self.body_height.saturating_sub(2).max(1) as isize;
                self.transcript.scroll_by(-page, self.max_scroll());
            }
            KeyCode::Tab if self.popup_open() => {
                self.accept_completion();
            }
            KeyCode::Esc if self.secret_for.is_some() => {
                self.secret_for = None;
                self.input.set("");
                self.transcript.push(views::info("key entry cancelled"));
            }
            KeyCode::Esc if self.setup.is_some() && !self.popup_open() => {
                self.input.set("");
                self.transcript.push(views::info("keeping the defaults"));
                self.finish_setup();
            }
            KeyCode::Esc => {
                if self.popup_open() {
                    self.dismissed = true;
                } else if self.transcript.scroll() > 0 {
                    self.transcript.scroll_to_bottom();
                } else {
                    self.input.set("");
                    self.edited();
                }
            }
            KeyCode::Enter => self.on_enter(),
            _ => {}
        }
    }

    fn on_enter(&mut self) {
        // Enter runs what was typed. It takes a suggestion only to finish a
        // partial command name or fixed-set value, during setup, or when
        // picked with the arrow keys; Tab always takes one.
        let completes = self.setup.is_some()
            || complete::enter_completes(&self.bank, self.input.text(), self.completion_context());
        if self.popup_open() && (self.chose || completes) {
            let typed = self.input.text().trim_end().to_string();
            let item = &self.completions[self.selected];
            // Accept a suggestion that changes the line; submit if it's final.
            if item.replacement.trim_end() != typed {
                match self.accept_completion() {
                    Some(true) => {}
                    _ => return,
                }
            }
        }
        if self.secret_for.is_some() {
            // Never echoed, never kept in history.
            let key = self.input.take_secret();
            self.completions.clear();
            self.transcript.push(views::input(
                &"•".repeat(key.trim().chars().count().min(24)),
            ));
            self.on_secret(key);
            return;
        }
        let line = self.input.take();
        self.completions.clear();
        let line = line.trim();
        if line.is_empty() && self.setup.is_some() {
            self.setup_text("");
            return;
        }
        if line.is_empty() {
            // Enter on an empty prompt does the obvious next thing.
            if let Some((command, _)) = self.default_action() {
                self.transcript.push(views::input(&command));
                self.execute(&command);
            }
            return;
        }
        if let Err(e) = self.store.push_history(line) {
            self.notify(format!("could not save history: {e}"));
        }
        self.transcript.push(views::input(line));
        self.execute(line);
    }

    /// `/past [question] [n]`: past attempts and their code. Without a
    /// question, the one being worked on.
    fn cmd_past(&self, args: &[&str]) -> Option<Entry> {
        let (q_args, n) = match args {
            [.., last] if args.len() > 1 && last.parse::<usize>().is_ok() => {
                (&args[..args.len() - 1], last.parse::<usize>().ok())
            }
            _ => (args, None),
        };
        let q = if q_args.is_empty() {
            match self
                .session
                .as_ref()
                .and_then(|s| s.queue.get(s.index))
                .and_then(|id| self.bank.get(*id))
            {
                Some(q) => q,
                None => return Some(views::error("which question?  ·  /past 11")),
            }
        } else {
            match self.resolve(q_args) {
                Ok(q) => q,
                Err(e) => return Some(e),
            }
        };
        let attempts = match self.store.past_attempts(q.meta.id) {
            Ok(a) => a,
            Err(e) => return Some(views::error(format!("could not read attempts: {e:#}"))),
        };
        if attempts.is_empty() {
            return Some(views::info(format!(
                "no attempts on #{} yet  ·  /solve {} to try it",
                q.meta.id, q.meta.id
            )));
        }
        // Default to the most recent attempt that has code.
        let shown = match n {
            Some(n) if (1..=attempts.len()).contains(&n) => n,
            Some(n) => {
                return Some(views::error(format!(
                    "#{} has {} attempt{}, not {n}",
                    q.meta.id,
                    attempts.len(),
                    if attempts.len() == 1 { "" } else { "s" }
                )));
            }
            None => attempts
                .iter()
                .rposition(|a| a.code.as_deref().is_some_and(|c| !c.trim().is_empty()))
                .map_or(attempts.len(), |i| i + 1),
        };
        Some(views::past(q, &attempts, shown))
    }

    /// `/donate`: opens the project's donation page.
    fn cmd_donate(&self) -> Entry {
        match crate::project::DONATE_URL {
            Some(url) => {
                let opened = crate::project::open_url(url).is_ok();
                views::donate(url, opened)
            }
            None => views::info(
                "Thanks for wanting to support dojo! Donations aren't set up yet.\nContributing questions and reporting bugs help a lot in the meantime.",
            ),
        }
    }

    /// Opens the report, optionally on one tag, company or difficulty.
    fn cmd_report(&mut self, args: &[&str]) -> Option<Entry> {
        let filter = args.first().map(|a| a.to_ascii_lowercase());
        if let Some(f) = &filter
            && !self.bank.labels().contains(f)
            && !["easy", "medium", "hard"].contains(&f.as_str())
        {
            return Some(views::error(format!(
                "no tag, company or difficulty `{f}`  ·  /list shows them"
            )));
        }
        match build_report(&self.store, &self.bank) {
            Ok(report) => {
                self.report = Some(ReportView::new(report, filter));
                None
            }
            Err(e) => Some(views::error(format!("could not build the report: {e:#}"))),
        }
    }

    fn on_report_key(&mut self, key: KeyEvent) {
        let Some(view) = self.report.as_mut() else {
            return;
        };
        let page = self.body_height.saturating_sub(2).max(1);
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.report = None,
            KeyCode::Char('c') | KeyCode::Char('d') if ctrl => self.report = None,
            KeyCode::Right | KeyCode::Tab | KeyCode::Char('l') => view.set_tab(view.tab.step(1)),
            KeyCode::Left | KeyCode::BackTab | KeyCode::Char('h') => {
                view.set_tab(view.tab.step(-1))
            }
            KeyCode::Char(c @ '1'..='5') => view.set_tab(Tab::ALL[c as usize - '1' as usize]),
            KeyCode::Down | KeyCode::Char('j') => view.scroll += 1,
            KeyCode::Up | KeyCode::Char('k') => view.scroll = view.scroll.saturating_sub(1),
            KeyCode::PageDown | KeyCode::Char(' ') => view.scroll += page,
            KeyCode::PageUp => view.scroll = view.scroll.saturating_sub(page),
            KeyCode::Home | KeyCode::Char('g') => view.scroll = 0,
            KeyCode::End | KeyCode::Char('G') => view.scroll = usize::MAX,
            _ => {}
        }
    }

    fn on_paste(&mut self, text: &str) {
        let flat = text.replace(['\r', '\n'], " ");
        self.input.insert(&flat);
        self.edited();
    }

    pub fn execute(&mut self, line: &str) {
        let Some(rest) = line.strip_prefix('/') else {
            if self.setup_text(line) || self.contrib_text(line) {
                return;
            }
            self.transcript.push(views::info(
                "Commands start with /  ·  try /help, or /show <id> to read a problem",
            ));
            return;
        };
        let mut parts = rest.split_whitespace();
        let name = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();
        let Some(spec) = commands::find(name) else {
            self.transcript.push(views::error(format!(
                "unknown command /{name}  ·  /help lists them"
            )));
            return;
        };
        if let Some(milestone) = spec.soon {
            self.transcript.push(views::warn(format!(
                "/{} isn't built yet — it lands in {milestone}. See PLAN.md.",
                spec.name
            )));
            return;
        }
        let out = match spec.name {
            "help" => self.cmd_help(&args),
            "list" => self.cmd_list(&args),
            "show" => self.cmd_show(&args),
            "solve" => self.cmd_solve(&args),
            "test" => self.run_tests(Which::Visible),
            "submit" => self.run_tests(Which::All),
            "hint" if args.is_empty() && self.session.is_some() => self.cmd_hint_session(),
            "hint" => self.cmd_hint(&args),
            "solution" if args.is_empty() && self.session.is_some() => self.cmd_solution_session(),
            "solution" => self.cmd_solution(&args),
            "skip" => self.cmd_skip(),
            "pause" => self.cmd_pause(),
            "edit" => self.cmd_edit(&args),
            "next" => self.cmd_next(),
            "editor" => self.cmd_editor(&args),
            "lang" => self.cmd_lang(&args),
            "config" => Some(views::config(&self.config, &self.paths)),
            "report" => self.cmd_report(&args),
            "contribute" => self.cmd_contribute(&args),
            "accept" => self.cmd_accept(),
            "past" => self.cmd_past(&args),
            "donate" => Some(self.cmd_donate()),
            "copy" => self.cmd_copy(),
            "clear" => {
                self.transcript.clear();
                None
            }
            "quit" => self.cmd_quit(),
            _ => Some(views::error(format!("/{} is not wired up", spec.name))),
        };
        if let Some(entry) = out {
            self.transcript.push(entry);
        }
    }

    /// Resolves an id, slug or search query to one question.
    fn resolve(&self, args: &[&str]) -> Result<&Question, Entry> {
        if args.is_empty() {
            return Err(views::error(
                "which question? give an id, slug or search words",
            ));
        }
        let query = args.join(" ");
        if let Ok(id) = query.parse::<u32>() {
            return self
                .bank
                .get(id)
                .ok_or_else(|| views::error(format!("no question #{id}  ·  /list to browse")));
        }
        if let Some(q) = self.bank.by_slug(&query) {
            return Ok(q);
        }
        match search(&self.bank, &query).first() {
            Some(q) => Ok(q),
            None => Err(views::error(format!("no question matches `{query}`"))),
        }
    }

    fn cmd_help(&self, args: &[&str]) -> Option<Entry> {
        match args.first() {
            None => Some(views::help(None)),
            Some(name) => match commands::find(name.trim_start_matches('/')) {
                Some(spec) => Some(views::help(Some(spec))),
                None => Some(views::error(format!("unknown command /{name}"))),
            },
        }
    }

    fn cmd_list(&self, args: &[&str]) -> Option<Entry> {
        let query = args.join(" ");
        Some(views::question_list(&search(&self.bank, &query), &query))
    }

    fn cmd_show(&self, args: &[&str]) -> Option<Entry> {
        // Plain `/show` in a session rereads the current question.
        if args.is_empty()
            && let Some(s) = &self.session
            && let Some(q) = s.queue.get(s.index).and_then(|id| self.bank.get(*id))
        {
            return Some(views::question(q));
        }
        Some(match self.resolve(args) {
            Ok(q) => views::question(q),
            Err(e) => e,
        })
    }

    fn cmd_hint(&self, args: &[&str]) -> Option<Entry> {
        // `/hint <id> [n]`; the session form (`/hint` alone) arrives in M2.
        let (q_args, n) = match args {
            [.., last] if args.len() > 1 && last.parse::<usize>().is_ok() => {
                (&args[..args.len() - 1], last.parse::<usize>().unwrap())
            }
            _ => (args, 1),
        };
        let q = match self.resolve(q_args) {
            Ok(q) => q,
            Err(e) => return Some(e),
        };
        if n == 0 || n > q.hints.len() {
            return Some(views::error(format!(
                "#{} has {} hints",
                q.meta.id,
                q.hints.len()
            )));
        }
        Some(views::hint(q, n, false))
    }

    fn cmd_solution(&mut self, args: &[&str]) -> Option<Entry> {
        let (id, entry) = match self.resolve(args) {
            Ok(q) => (q.meta.id, views::solution(q, self.config.lang())),
            Err(e) => return Some(e),
        };
        self.explained_now(id);
        Some(entry)
    }

    /// Notes that a question's explained solution was shown.
    pub(super) fn explained_now(&mut self, id: u32) {
        if !self.explained.contains(&id) {
            self.explained.push(id);
        }
        if let Some((review, _)) = &mut self.review {
            review.retain(|r| *r != id);
        }
    }

    /// `/quit`: when questions went unsolved, offers their explained
    /// solutions first (once); `/quit` again leaves.
    fn cmd_quit(&mut self) -> Option<Entry> {
        if self.review.is_none() {
            let unsolved = self
                .session
                .as_ref()
                .map(|s| missed(&s.done, &self.explained))
                .unwrap_or_default();
            if !unsolved.is_empty() {
                self.end_session(true);
                let offer = views::review_offer(&self.bank, &unsolved);
                self.review = Some((unsolved.iter().map(|(id, _)| *id).collect(), unsolved.len()));
                return Some(offer);
            }
        }
        self.end_session(false);
        self.quit = true;
        None
    }

    fn save_config(&mut self) -> Option<Entry> {
        match self.config.save(&self.paths.config_file) {
            Ok(()) => None,
            Err(e) => Some(views::error(format!("could not save config: {e:#}"))),
        }
    }

    fn cmd_editor(&mut self, args: &[&str]) -> Option<Entry> {
        if args.is_empty() {
            let (editor, source) = self.config.editor();
            return Some(views::info(format!(
                "editor: {editor} ({source})  ·  /editor <name> to change: {}",
                crate::config::EDITOR_PRESETS.join(", ")
            )));
        }
        let raw = args.join(" ");
        let command = editor_preset(&raw).map(str::to_string).unwrap_or(raw);
        let program = command.split_whitespace().next().unwrap_or_default();
        let found = which(program);
        self.config.editor = Some(command.clone());
        if let Some(err) = self.save_config() {
            return Some(err);
        }
        Some(if found {
            views::ok(format!("editor set to `{command}`"))
        } else {
            views::warn(format!(
                "editor set to `{command}`, but `{program}` isn't on your PATH"
            ))
        })
    }

    fn cmd_lang(&mut self, args: &[&str]) -> Option<Entry> {
        let Some(name) = args.first() else {
            let all: Vec<&str> = Language::ALL.iter().map(|l| l.name()).collect();
            return Some(views::info(format!(
                "language: {}  ·  available: {}",
                self.config.language,
                all.join(", ")
            )));
        };
        let Some(lang) = Language::parse(name) else {
            return Some(views::error(format!(
                "`{name}` isn't supported  ·  {}",
                Language::ALL
                    .iter()
                    .map(|l| l.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        };
        self.config.language = lang.name().to_string();
        if let Some(err) = self.save_config() {
            return Some(err);
        }
        Some(match crate::runner::toolchain(lang) {
            Ok(v) => views::ok(format!("language set to {lang} ({v})")),
            Err(e) => views::warn(format!("language set to {lang}, but {e:#}")),
        })
    }

    fn cmd_copy(&mut self) -> Option<Entry> {
        let Some(text) = self.transcript.last_copy().map(str::to_string) else {
            return Some(views::info("nothing to copy yet"));
        };
        let lines = text.lines().count();
        Some(match ui::clipboard::copy(&text) {
            Ok(ui::clipboard::Copied::Native) => {
                self.notify(format!("copied {lines} lines"));
                return None;
            }
            Ok(ui::clipboard::Copied::Terminal) => {
                self.notify(format!(
                    "sent {lines} lines to the clipboard through the terminal (OSC 52)"
                ));
                return None;
            }
            Err(e) => views::error(format!("copy failed: {e}")),
        })
    }
}

/// Questions that ended failed or skipped, once each, minus those already
/// explained.
fn missed(done: &[Done], explained: &[u32]) -> Vec<(u32, Outcome)> {
    let mut out: Vec<(u32, Outcome)> = Vec::new();
    for d in done {
        let solved_later = done.iter().any(|o| {
            o.question_id == d.question_id && matches!(o.outcome, Outcome::Pass | Outcome::Revealed)
        });
        if matches!(d.outcome, Outcome::Fail | Outcome::Skip)
            && !solved_later
            && !explained.contains(&d.question_id)
            && !out.iter().any(|(id, _)| *id == d.question_id)
        {
            out.push((d.question_id, d.outcome));
        }
    }
    out
}

fn which(program: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| dir.join(program).is_file())
}

pub fn run(initial: Option<String>) -> Result<()> {
    use std::io::{IsTerminal, Write};
    // Drawing into a pipe or a dumb terminal would only write escape codes.
    if !std::io::stdout().is_terminal() {
        anyhow::bail!(
            "dojo needs a terminal to open in (stdout isn't one)  ·  for scripts use \
             `dojo report --json` or `dojo validate`"
        );
    }
    if std::env::var_os("TERM").is_some_and(|t| t == "dumb") {
        anyhow::bail!(
            "dojo can't draw in a dumb terminal (TERM=dumb)  ·  run it in a full terminal"
        );
    }
    let mut app = App::new()?;
    let mut term = ui::terminal::init()?;
    if let Some(line) = initial {
        app.transcript.push(views::input(&line));
        app.execute(&line);
    }

    let result = (|| -> Result<()> {
        while !app.quit {
            if let Some((command, after)) = app.pending_command.take() {
                ui::terminal::suspend()?;
                let result = crate::session::launch_terminal(&command);
                ui::terminal::resume(&mut term)?;
                match after {
                    AfterCommand::Editor => app.editor_closed(result),
                    other => app.after_setup(other, result.unwrap_or(false)),
                }
            }
            while let Ok(msg) = app.rx.try_recv() {
                match msg {
                    Msg::Contrib(m) => app.on_contrib(m),
                    Msg::Tests {
                        attempt_id,
                        run,
                        result,
                    } => {
                        if run == app.run_seq && app.running.is_some() {
                            app.on_tests(attempt_id, result);
                        }
                    }
                }
            }
            app.check_saved();
            app.autosave();
            app.check_idle();

            term.draw(|f| ui::draw(f, &mut app))?;
            // Wake often enough for the timer and spinner.
            let tick = if app.running.is_some() { 80 } else { 250 };
            if !event::poll(Duration::from_millis(tick))? {
                continue;
            }
            // Drain everything queued before redrawing (fast typing, pastes).
            loop {
                let ev = event::read()?;
                if matches!(ev, Event::Key(_) | Event::Paste(_) | Event::Mouse(_)) {
                    app.active();
                }
                match ev {
                    Event::Key(key) => app.on_key(key),
                    Event::Paste(text) => app.on_paste(&text),
                    Event::Mouse(m) => match (m.kind, app.report.as_mut()) {
                        (MouseEventKind::ScrollUp, Some(view)) => {
                            view.scroll = view.scroll.saturating_sub(3)
                        }
                        (MouseEventKind::ScrollDown, Some(view)) => view.scroll += 3,
                        (MouseEventKind::ScrollUp, None) => {
                            app.transcript.scroll_by(3, app.max_scroll())
                        }
                        (MouseEventKind::ScrollDown, None) => {
                            app.transcript.scroll_by(-3, app.max_scroll())
                        }
                        _ => {}
                    },
                    _ => {}
                }
                if app.quit || app.pending_command.is_some() || !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }
        Ok(())
    })();

    if result.is_err() {
        app.end_session(false);
    }
    ui::terminal::restore();
    let printed = match app.last_summary.as_ref().filter(|d| !d.is_empty()) {
        Some(done) => writeln!(
            std::io::stdout().lock(),
            "{}",
            views::summary_text(&app.bank, done)
        ),
        None => Ok(()),
    };
    result?;
    Ok(printed?)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rstest::rstest;

    use super::*;

    fn done(id: u32, outcome: Outcome) -> Done {
        Done {
            question_id: id,
            outcome,
            elapsed: Duration::ZERO,
            hints: 0,
            runs: 0,
            failed_runs: 0,
        }
    }

    #[rstest]
    #[case::fails_and_skips(
        &[(1, Outcome::Pass), (2, Outcome::Fail), (3, Outcome::Skip), (4, Outcome::Revealed)],
        &[],
        &[(2, Outcome::Fail), (3, Outcome::Skip)]
    )]
    #[case::unfinished_isnt_spoiled(&[(5, Outcome::Unfinished)], &[], &[])]
    #[case::already_explained(&[(2, Outcome::Fail), (3, Outcome::Skip)], &[2], &[(3, Outcome::Skip)])]
    #[case::solved_on_a_retry(&[(2, Outcome::Fail), (2, Outcome::Pass)], &[], &[])]
    #[case::once_each(&[(2, Outcome::Skip), (2, Outcome::Fail)], &[], &[(2, Outcome::Skip)])]
    fn picks_what_to_review(
        #[case] session: &[(u32, Outcome)],
        #[case] explained: &[u32],
        #[case] expected: &[(u32, Outcome)],
    ) {
        let session: Vec<Done> = session.iter().map(|(id, o)| done(*id, *o)).collect();
        assert_eq!(missed(&session, explained), expected);
    }
}
