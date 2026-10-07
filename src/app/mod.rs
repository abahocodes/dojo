//! The fullscreen app: state, event loop and command dispatch.

mod commands;
mod complete;
mod input;
mod practice;
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

pub use complete::{Item, MAX_ITEMS};
pub use transcript::{Entry, Transcript};

const HISTORY_LIMIT: usize = 500;

/// Guidance for the empty prompt.
pub struct Prompt {
    /// The command Enter runs, and its label.
    pub enter: Option<(String, String)>,
    /// Other things that make sense now: (key, description).
    pub tips: Vec<(String, String)>,
}

/// Messages from background work.
pub enum Msg {
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
    /// Terminal editor command waiting for the main loop to hand it the
    /// terminal.
    pending_editor: Option<String>,
    /// When the open attempt's clock was last saved.
    last_autosave: Instant,
    /// Unfinished attempts, most recent first (refreshed when sessions end).
    pub unfinished: Vec<crate::store::OpenAttempt>,
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
        transcript.push(views::welcome(&bank, &config, &unfinished));
        let (tx, rx) = channel();
        Ok(App {
            bank,
            config,
            paths,
            store,
            transcript,
            input: input::Input::with_history(history),
            completions: vec![],
            selected: 0,
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
            pending_editor: None,
            last_autosave: Instant::now(),
            unfinished,
            last_summary: None,
            quit: false,
        })
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

        if self.running.is_some() {
            return Prompt {
                enter: None,
                tips: vec![tip("", "running tests…"), tip("Ctrl+C", "cancels")],
            };
        }
        let Some(s) = &self.session else {
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

    fn refresh_completions(&mut self) {
        let ctx = complete::Context {
            attempt: self.session.as_ref().is_some_and(|s| s.attempt.is_some()),
            session: self.session.is_some(),
        };
        self.completions = complete::complete(&self.bank, self.input.text(), ctx);
        self.selected = 0;
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
            }
            KeyCode::Down if self.popup_open() => {
                let n = self.completions.len().min(complete::MAX_ITEMS);
                self.selected = (self.selected + 1) % n;
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
        if self.popup_open() {
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
        let line = self.input.take();
        self.completions.clear();
        let line = line.trim();
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

    fn on_paste(&mut self, text: &str) {
        let flat = text.replace(['\r', '\n'], " ");
        self.input.insert(&flat);
        self.edited();
    }

    pub fn execute(&mut self, line: &str) {
        let Some(rest) = line.strip_prefix('/') else {
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
            "copy" => self.cmd_copy(),
            "clear" => {
                self.transcript.clear();
                None
            }
            "quit" => {
                self.end_session(false);
                self.quit = true;
                None
            }
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

    fn cmd_solution(&self, args: &[&str]) -> Option<Entry> {
        Some(match self.resolve(args) {
            Ok(q) => views::solution(q, self.config.lang()),
            Err(e) => e,
        })
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
                "`{name}` isn't supported  ·  python or javascript"
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
        Some(match ui::clipboard::copy(&text) {
            Ok(()) => {
                self.notify(format!("copied {} lines", text.lines().count()));
                return None;
            }
            Err(e) => views::error(format!("copy failed: {e}")),
        })
    }
}

fn which(program: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| dir.join(program).is_file())
}

pub fn run(initial: Option<String>) -> Result<()> {
    let mut app = App::new()?;
    let mut term = ui::terminal::init()?;
    if let Some(line) = initial {
        app.transcript.push(views::input(&line));
        app.execute(&line);
    }

    let result = (|| -> Result<()> {
        while !app.quit {
            if let Some(command) = app.pending_editor.take() {
                ui::terminal::suspend()?;
                let result = crate::session::launch_terminal(&command);
                ui::terminal::resume(&mut term)?;
                app.editor_closed(result);
            }
            while let Ok(msg) = app.rx.try_recv() {
                match msg {
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

            term.draw(|f| ui::draw(f, &mut app))?;
            // Wake often enough for the timer and spinner.
            let tick = if app.running.is_some() { 80 } else { 250 };
            if !event::poll(Duration::from_millis(tick))? {
                continue;
            }
            // Drain everything queued before redrawing (fast typing, pastes).
            loop {
                match event::read()? {
                    Event::Key(key) => app.on_key(key),
                    Event::Paste(text) => app.on_paste(&text),
                    Event::Mouse(m) => match m.kind {
                        MouseEventKind::ScrollUp => app.transcript.scroll_by(3, app.max_scroll()),
                        MouseEventKind::ScrollDown => {
                            app.transcript.scroll_by(-3, app.max_scroll())
                        }
                        _ => {}
                    },
                    _ => {}
                }
                if app.quit || app.pending_editor.is_some() || !event::poll(Duration::ZERO)? {
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
    if let Some(done) = app.last_summary.as_ref().filter(|d| !d.is_empty()) {
        println!("{}", views::summary_text(&app.bank, done));
    }
    result
}
