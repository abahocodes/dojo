//! Practice commands: /solve, /test, /submit, /hint, /solution, /edit,
//! /pause, /skip, /next — plus the background test runner, the save
//! watcher and autosave.
//!
//! A session is the period the dojo window is open: it starts with the first
//! question and ends when dojo closes.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// How often an open attempt's clock is saved.
const AUTOSAVE_EVERY: Duration = Duration::from_secs(5);

use serde_json::json;

use super::views;
use super::{App, Entry, Msg};
use crate::questions::Question;
use crate::questions::search::search;
use crate::runner::{self, Language, Which};
use crate::session::{
    self, Attempt, Done, Outcome, Session, Timer, editor_command, is_terminal_editor,
};
use crate::store::{AttemptRow, OpenAttempt};

impl App {
    fn current_question(&self) -> Option<&Question> {
        let s = self.session.as_ref()?;
        self.bank.get(*s.queue.get(s.index)?)
    }

    fn record(&mut self, kind: &str, payload: serde_json::Value) {
        let Some(a) = self.session.as_ref().and_then(|s| s.attempt.as_ref()) else {
            return;
        };
        if let Err(e) = self.store.event(a.id, kind, payload) {
            self.notify(format!("could not record {kind}: {e:#}"));
        }
    }

    fn save_attempt(&mut self, outcome: Option<Outcome>, code: Option<&str>) {
        let Some(a) = self.session.as_ref().and_then(|s| s.attempt.as_ref()) else {
            return;
        };
        let row = AttemptRow::builder()
            .id(a.id)
            .active_secs(a.timer.elapsed().as_secs())
            .test_runs(a.test_runs)
            .failed_runs(a.failed_runs)
            .hints_used(a.hints_used)
            .solution_viewed(a.solution_viewed)
            .maybe_outcome(outcome.map(Outcome::as_str))
            .maybe_code(code)
            .build();
        if let Err(e) = self.store.save_attempt(&row) {
            self.notify(format!("could not save attempt: {e:#}"));
        }
    }

    pub(super) fn cmd_solve(&mut self, args: &[&str]) -> Option<Entry> {
        let lang = self.config.lang();
        if let Err(e) = runner::toolchain(lang) {
            return Some(views::error(format!("{e:#}")));
        }

        // Parse `-n N`; the rest is ids, `random [N]`, or a search query.
        let mut n: Option<usize> = None;
        let mut words: Vec<&str> = Vec::new();
        let mut it = args.iter();
        while let Some(&a) = it.next() {
            if a == "-n" {
                match it.next().and_then(|v| v.parse().ok()) {
                    Some(v) if v > 0 => n = Some(v),
                    _ => return Some(views::error("-n needs a positive number")),
                }
            } else {
                words.push(a);
            }
        }
        if words.is_empty() {
            return Some(views::error(
                "what should we practice?  ·  /solve 12, /solve 3 7 9, /solve graphs -n 2",
            ));
        }

        // `random` and `need` take an optional count: `/solve need 3`.
        let count = |keyword: &str| match (words.get(1), n) {
            (Some(w), _) => match w.parse::<usize>() {
                Ok(v) if v > 0 => Ok(v),
                _ => Err(views::error(format!("usage: /solve {keyword} [N]"))),
            },
            (None, Some(v)) => Ok(v),
            (None, None) => Ok(1),
        };
        let (queue, mode, query): (Vec<u32>, &str, Option<String>) = if words[0] == "need" {
            let count = match count("need") {
                Ok(c) => c,
                Err(e) => return Some(e),
            };
            let report = match super::build_report(&self.store, &self.bank) {
                Ok(r) => r,
                Err(e) => return Some(views::error(format!("could not read your history: {e:#}"))),
            };
            let picks: Vec<crate::model::Suggestion> = crate::model::suggest()
                .questions(&report.questions)
                .topics(&report.topics)
                .now(jiff::Timestamp::now())
                .limit(usize::MAX)
                .call()
                .into_iter()
                .filter(|s| {
                    self.bank
                        .get(s.question_id)
                        .is_some_and(|q| q.supports(lang))
                })
                .take(count)
                .collect();
            if picks.is_empty() {
                return Some(views::info(
                    "nothing to suggest yet  ·  /solve random to start",
                ));
            }
            self.transcript.push(views::need_picks(&picks));
            (picks.iter().map(|p| p.question_id).collect(), "need", None)
        } else if words[0] == "random" {
            let count = match count("random") {
                Ok(c) => c,
                Err(e) => return Some(e),
            };
            let pool: Vec<u32> = self
                .bank
                .all()
                .iter()
                .filter(|q| q.supports(lang))
                .map(|q| q.meta.id)
                .collect();
            (pick_random(pool, count), "random", None)
        } else if words.iter().all(|w| w.parse::<u32>().is_ok()) {
            let ids: Vec<u32> = words.iter().map(|w| w.parse().unwrap()).collect();
            if let Some(missing) = ids.iter().find(|id| self.bank.get(**id).is_none()) {
                return Some(views::error(format!("no question #{missing}  ·  /list")));
            }
            if let Some(q) = ids
                .iter()
                .filter_map(|id| self.bank.get(*id))
                .find(|q| !q.supports(lang))
            {
                return Some(views::error(format!(
                    "#{} isn't available in {} yet  ·  /lang to switch",
                    q.meta.id,
                    lang.label()
                )));
            }
            (ids, "id", None)
        } else {
            let query = words.join(" ");
            let found: Vec<u32> = match self.bank.by_slug(&query) {
                Some(q) => vec![q.meta.id],
                None => search(&self.bank, &query)
                    .into_iter()
                    .filter(|q| q.supports(lang))
                    .take(n.unwrap_or(1))
                    .map(|q| q.meta.id)
                    .collect(),
            };
            if found.is_empty() {
                return Some(views::error(format!("no question matches `{query}`")));
            }
            (found, "search", Some(query))
        };
        if let Some(n) = n
            && queue.len() < n
        {
            self.notify(format!("only {} question(s) match", queue.len()));
        }

        // Starting something else skips whatever is open (the selection
        // above is already validated, so a typo never skips anything).
        if let Some(a) = self.session.as_ref().and_then(|s| s.attempt.as_ref()) {
            let outcome = skip_outcome(a.test_runs);
            let (qid, title) = (
                a.question_id,
                self.bank
                    .get(a.question_id)
                    .map(|q| q.meta.title.clone())
                    .unwrap_or_default(),
            );
            self.finish(outcome, false);
            self.transcript
                .push(views::info(format!("↷ skipped #{qid} {title} ({outcome})")));
        }

        // One session per dojo window: the first /solve starts it, later ones
        // queue more questions into it.
        match self.session.as_mut() {
            Some(s) => {
                s.queue = queue;
                s.index = 0;
                s.mode = mode.to_string();
                s.query = query;
            }
            None => {
                let id = match self.store.create_session("practice", None) {
                    Ok(id) => id,
                    Err(e) => {
                        return Some(views::error(format!("could not start session: {e:#}")));
                    }
                };
                self.review = None;
                self.session = Some(
                    Session::builder()
                        .id(id)
                        .mode(mode.to_string())
                        .maybe_query(query)
                        .queue(queue)
                        .build(),
                );
            }
        }
        self.start_question(lang, None)
    }

    /// Opens the current queue question: a fresh attempt from the
    /// boilerplate (`/solve`), or `continuing` an unfinished one (`/edit`).
    fn start_question(&mut self, lang: Language, continuing: Option<OpenAttempt>) -> Option<Entry> {
        let q = self.current_question()?.clone();
        let session = self.session.as_ref()?;
        let position = (session.index, session.queue.len());
        let session_id = session.id;
        let selection = json!({ "mode": session.mode, "query": session.query });
        let stats = self.store.question_stats(q.meta.id).unwrap_or_default();

        // /solve always starts fresh: an unfinished attempt on this question
        // is closed (its code stays in the database).
        if continuing.is_none() {
            match self.store.open_attempt(q.meta.id, lang.name()) {
                Ok(Some(old)) => {
                    self.close_open_attempt(&old);
                    self.transcript.push(views::info(format!(
                        "↷ skipped your unfinished attempt on #{} from {} (its code is saved)",
                        q.meta.id,
                        views::ago(&old.started_at)
                    )));
                }
                Ok(None) => {}
                Err(e) => return Some(views::error(format!("could not read attempts: {e:#}"))),
            }
        }
        let open = continuing;

        let workspace = self.config.workspace(&self.paths);
        let saved = open.as_ref().and_then(|o| o.code.as_deref());
        let fresh = open.is_none();
        let prepared = session::prepare_work()
            .root(&workspace)
            .question(&q)
            .lang(lang)
            .maybe_saved(saved)
            .fresh(fresh)
            .call();
        let (dir, file, _) = match prepared {
            Ok(v) => v,
            Err(e) => return Some(views::error(format!("{e:#}"))),
        };

        let (attempt, resume) = match &open {
            // Continue the same attempt: same row, same clock, same counters.
            Some(o) => (
                Attempt::builder()
                    .id(o.id)
                    .question_id(q.meta.id)
                    .lang(lang)
                    .dir(dir)
                    .file(file)
                    .timer(Timer::resume_from(Duration::from_secs(
                        o.active_secs.max(0) as u64,
                    )))
                    .test_runs(o.test_runs as u32)
                    .failed_runs(o.failed_runs as u32)
                    .hints_used(o.hints_used as usize)
                    .solution_viewed(o.solution_viewed)
                    .solution_armed(o.solution_viewed)
                    .build(),
                views::Resume::Continued {
                    elapsed: Duration::from_secs(o.active_secs.max(0) as u64),
                    started_at: o.started_at.clone(),
                },
            ),
            None => {
                let id = match self
                    .store
                    .create_attempt(session_id, q.meta.id, lang.name())
                {
                    Ok(id) => id,
                    Err(e) => {
                        return Some(views::error(format!("could not record attempt: {e:#}")));
                    }
                };
                (
                    Attempt::builder()
                        .id(id)
                        .question_id(q.meta.id)
                        .lang(lang)
                        .dir(dir)
                        .file(file)
                        .timer(Timer::start())
                        .build(),
                    views::Resume::Fresh,
                )
            }
        };

        let entry = views::session_question()
            .question(&q)
            .position(position)
            .file(&attempt.file)
            .stats(&stats)
            .resume(&resume)
            .call();
        if let Some(s) = self.session.as_mut() {
            s.attempt = Some(attempt);
        }
        match open {
            Some(_) => self.record("continue", json!({ "session_id": session_id })),
            None => self.record(
                "start",
                json!({ "language": lang.name(), "selection": selection }),
            ),
        }
        self.last_autosave = Instant::now();
        self.transcript.push(entry);
        self.open_editor()
    }

    /// Continues an unfinished attempt (the most recent, or the one on
    /// `target`): same attempt, same clock, in the language it was started in.
    fn continue_unfinished(&mut self, target: Option<u32>) -> Option<Entry> {
        self.refresh_unfinished();
        let found = self
            .unfinished
            .iter()
            .find(|o| target.is_none_or(|id| o.question_id == id as i64))
            .cloned();
        let Some(o) = found else {
            return Some(views::info(match target {
                Some(id) => format!("no unfinished attempt on #{id}  ·  /solve {id} to start it"),
                None => "nothing to continue  ·  /solve <id> to start".into(),
            }));
        };
        let qid = o.question_id as u32;
        let Some(lang) = Language::parse(&o.language) else {
            return Some(views::error(format!("unknown language `{}`", o.language)));
        };
        if self.bank.get(qid).is_none_or(|q| !q.supports(lang)) {
            return Some(views::error(format!(
                "#{qid} is no longer available in {}",
                lang.label()
            )));
        }
        if let Err(e) = runner::toolchain(lang) {
            return Some(views::error(format!("{e:#}")));
        }
        match self.session.as_mut() {
            Some(s) => {
                s.queue = vec![qid];
                s.index = 0;
                s.mode = "continue".into();
                s.query = None;
            }
            None => {
                let id = match self.store.create_session("practice", None) {
                    Ok(id) => id,
                    Err(e) => return Some(views::error(format!("could not start session: {e:#}"))),
                };
                self.review = None;
                self.session = Some(
                    Session::builder()
                        .id(id)
                        .mode("continue".into())
                        .queue(vec![qid])
                        .build(),
                );
            }
        }
        self.start_question(lang, Some(o))
    }

    pub(super) fn refresh_unfinished(&mut self) {
        if let Ok(open) = self.store.open_attempts() {
            self.unfinished = open;
        }
    }

    /// Records an unfinished attempt from an earlier window as skipped
    /// (`/solve` started that question over).
    fn close_open_attempt(&mut self, o: &crate::store::OpenAttempt) {
        let row = AttemptRow::builder()
            .id(o.id)
            .active_secs(o.active_secs.max(0) as u64)
            .test_runs(o.test_runs as u32)
            .failed_runs(o.failed_runs as u32)
            .hints_used(o.hints_used as usize)
            .solution_viewed(o.solution_viewed)
            .outcome(skip_outcome(o.test_runs as u32).as_str())
            .build();
        let result = self.store.save_attempt(&row).and_then(|_| {
            self.store.event(
                o.id,
                "outcome",
                json!({ "outcome": skip_outcome(o.test_runs as u32).as_str() }),
            )
        });
        if let Err(e) = result {
            self.notify(format!("could not close the old attempt: {e:#}"));
        }
    }

    /// Saves the open attempt's clock now and then, so a crash or a closed
    /// terminal loses at most a few seconds.
    pub(super) fn autosave(&mut self) {
        let open = self.session.as_ref().is_some_and(|s| s.attempt.is_some());
        if open && self.last_autosave.elapsed() >= AUTOSAVE_EVERY {
            self.last_autosave = Instant::now();
            self.save_attempt(None, None);
        }
    }

    /// Opens the solution in the user's editor. Terminal editors are queued
    /// for the main loop, which hands over the terminal.
    fn open_editor(&mut self) -> Option<Entry> {
        let a = self.session.as_mut()?.attempt.as_mut()?;
        a.mtime = a.file_mtime();
        // Put the cursor where the user left off.
        let code = std::fs::read_to_string(&a.file).unwrap_or_default();
        let line = self
            .bank
            .get(a.question_id)
            .and_then(|q| q.boilerplate.get(&a.lang))
            .map_or(1, |b| session::resume_line(b, &code));
        let (template, _) = self.config.editor();
        let command = editor_command()
            .template(&template)
            .file(&a.file)
            .dir(&a.dir)
            .line(line)
            .call();
        self.record("editor_open", json!({ "command": template }));
        if is_terminal_editor(&template) {
            self.pending_command = Some((command, super::AfterCommand::Editor));
            None
        } else {
            let log = self
                .paths
                .default_workspace
                .parent()
                .unwrap_or(&self.paths.default_workspace)
                .join("editor.log");
            match session::launch_gui(&command, &log) {
                Ok(()) => {
                    let program = template.split_whitespace().next().unwrap_or("editor");
                    Some(views::info(format!(
                        "opened in {program}  ·  tests re-run each time you save"
                    )))
                }
                Err(e) => Some(views::error(format!(
                    "{e:#}  ·  set one with /editor (e.g. /editor nvim)"
                ))),
            }
        }
    }

    /// The user did something (a key, a paste, a save, closing the editor).
    /// If the timer paused itself while they were away, it resumes.
    pub(super) fn active(&mut self) {
        self.last_activity = Instant::now();
        let Some(since) = self.away.take() else {
            return;
        };
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        if a.timer.resume() {
            self.record(
                "resume",
                json!({ "after_idle_secs": since.elapsed().as_secs() }),
            );
            self.save_attempt(None, None);
            self.transcript.push(views::info(format!(
                "▶ welcome back  ·  you were away {}, which isn't counted",
                away_for(since.elapsed())
            )));
        }
    }

    /// Pauses the timer when nothing has happened for a while: no key in
    /// dojo and no save. The idle time isn't counted.
    pub(super) fn check_idle(&mut self) {
        let minutes = self.config.idle_pause_minutes;
        if minutes == 0 || self.running.is_some() || self.away.is_some() {
            return;
        }
        let idle = self.last_activity.elapsed();
        if idle < Duration::from_secs(minutes * 60) {
            return;
        }
        let last_seen = self.last_activity;
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        if !a.timer.pause_at(last_seen) {
            return;
        }
        self.away = Some(last_seen);
        self.record("idle_pause", json!({ "idle_secs": idle.as_secs() }));
        self.save_attempt(None, None);
        self.transcript.push(views::info(format!(
            "⏸ paused: nothing for {minutes} min, so that time isn't counted  ·  press any key or save to carry on"
        )));
    }

    /// Called after a terminal editor exits.
    pub(super) fn editor_closed(&mut self, result: anyhow::Result<bool>) {
        self.active();
        match result {
            Ok(true) => {}
            Ok(false) => self
                .transcript
                .push(views::warn("editor exited with an error")),
            Err(e) => self
                .transcript
                .push(views::error(format!("{e:#}  ·  set one with /editor"))),
        }
        self.check_saved();
    }

    /// Re-runs visible tests when the solution file changed on disk.
    pub(super) fn check_saved(&mut self) {
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        let now = a.file_mtime();
        // A save while away counts as being back.
        if self.away.is_some() && now.is_some() && now != a.mtime {
            self.active();
        }
        if self.running.is_some() || !self.config.auto_test {
            return;
        }
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        if a.timer.paused() {
            return;
        }
        if now.is_some() && now != a.mtime {
            a.mtime = now;
            self.transcript.push(views::input("/test  (saved)"));
            self.run_tests(Which::Visible);
        }
    }

    /// Back to work: restarts a paused timer and reopens the editor where
    /// the user left off.
    pub(super) fn cmd_edit(&mut self, args: &[&str]) -> Option<Entry> {
        let a = match self.session.as_mut().and_then(|s| s.attempt.as_mut()) {
            Some(a) => a,
            // Nothing open: continue unfinished work.
            None => {
                let target = match args.first().map(|a| a.parse::<u32>()) {
                    None => None,
                    Some(Ok(id)) => Some(id),
                    Some(Err(_)) => return Some(views::error("usage: /edit [id]")),
                };
                return self.continue_unfinished(target);
            }
        };
        if a.timer.resume() {
            a.mtime = a.file_mtime(); // edits made while paused aren't a save
            self.record("resume", serde_json::Value::Null);
            self.save_attempt(None, None);
            self.transcript.push(views::info("▶ timer running again"));
        }
        self.open_editor()
    }

    fn attempt_or_err(&self) -> Result<&Attempt, Entry> {
        self.session
            .as_ref()
            .and_then(|s| s.attempt.as_ref())
            .ok_or_else(|| views::error("no question in progress  ·  /solve <id> to start"))
    }

    pub(super) fn run_tests(&mut self, which: Which) -> Option<Entry> {
        let a = match self.attempt_or_err() {
            Ok(a) => a,
            Err(e) => return Some(e),
        };
        if self.running.is_some() {
            self.notify("tests are already running");
            return None;
        }
        let q = self.bank.get(a.question_id)?.clone();
        let (lang, file, attempt_id) = (a.lang, a.file.clone(), a.id);
        let tx = self.tx.clone();
        self.run_seq += 1;
        let run = self.run_seq;
        let cancel = Arc::new(AtomicBool::new(false));
        self.cancel = Some(cancel.clone());
        std::thread::spawn(move || {
            let result = runner::run()
                .question(&q)
                .lang(lang)
                .solution(&file)
                .which(which)
                .cancel(&cancel)
                .call()
                .map_err(|e| format!("{e:#}"));
            let _ = tx.send(Msg::Tests {
                attempt_id,
                run,
                result,
            });
        });
        self.running = Some((which, Instant::now()));
        None
    }

    /// Stops the test run in flight; its results are discarded.
    pub(super) fn cancel_tests(&mut self) {
        if let Some(flag) = self.cancel.take() {
            flag.store(true, Ordering::Relaxed);
        }
        if self.running.take().is_some() {
            self.transcript.push(views::info("test run cancelled"));
        }
    }

    pub(super) fn on_tests(&mut self, attempt_id: i64, result: Result<runner::RunReport, String>) {
        self.running = None;
        let report = match result {
            Ok(r) => r,
            Err(e) => {
                self.transcript
                    .push(views::error(format!("could not run tests: {e}")));
                return;
            }
        };
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        if a.id != attempt_id {
            return; // a stale run from a previous question
        }
        a.test_runs += 1;
        let passed = report.all_passed();
        if !passed {
            a.failed_runs += 1;
        }
        let expected = q_case_count(&self.bank, a.question_id, report.which);
        a.last_run = Some((report.passed(), report.total().max(expected)));
        let which = report.which;
        let Some(q) = self.bank.get(a.question_id).cloned() else {
            return;
        };

        self.record(
            "test_run",
            json!({
                "which": if which == Which::All { "all" } else { "visible" },
                "passed": report.passed(),
                "total": report.total(),
                "fatal": report.fatal.is_some(),
            }),
        );
        self.save_attempt(None, None);
        self.transcript.push(views::test_report(&q, &report));

        if passed && which == Which::All {
            let viewed = self
                .session
                .as_ref()
                .and_then(|s| s.attempt.as_ref())
                .is_some_and(|a| a.solution_viewed);
            self.finish(
                if viewed {
                    Outcome::Revealed
                } else {
                    Outcome::Pass
                },
                true,
            );
        } else if passed {
            self.notify("visible tests pass  ·  /submit to run hidden ones");
        }
    }

    /// Closes the current attempt with an outcome; `show` prints the result
    /// card with what to do next.
    fn finish(&mut self, outcome: Outcome, show: bool) {
        let Some(a) = self.session.as_ref().and_then(|s| s.attempt.as_ref()) else {
            return;
        };
        let code = std::fs::read_to_string(&a.file).ok();
        let work_dir = a.dir.clone();
        let done = Done::builder()
            .question_id(a.question_id)
            .outcome(outcome)
            .elapsed(a.timer.elapsed())
            .hints(a.hints_used)
            .runs(a.test_runs)
            .failed_runs(a.failed_runs)
            .build();
        self.record("outcome", json!({ "outcome": outcome.as_str() }));
        self.notice = None;
        self.save_attempt(Some(outcome), code.as_deref());
        // The attempt is over and its code is in the database.
        session::clear_work(&work_dir);

        let Some(s) = self.session.as_mut() else {
            return;
        };
        s.attempt = None;
        s.done.push(done.clone());
        let next = s.queue.get(s.index + 1).copied();
        if !show {
            return;
        }
        if let Some(q) = self.bank.get(done.question_id) {
            let next = next.and_then(|id| self.bank.get(id));
            self.transcript.push(views::finished(q, &done, next));
        }
    }

    pub(super) fn cmd_hint_session(&mut self) -> Option<Entry> {
        let q = self.current_question()?.clone();
        let a = self.session.as_mut()?.attempt.as_mut();
        let Some(a) = a else {
            // Between questions: browsing hints doesn't count.
            return Some(views::hint(&q, 1, false));
        };
        if a.hints_used >= q.hints.len() {
            return Some(views::info(format!(
                "that was the last hint ({})  ·  /solution to see the answer",
                q.hints.len()
            )));
        }
        a.hints_used += 1;
        let n = a.hints_used;
        self.record("hint", json!({ "n": n }));
        self.save_attempt(None, None);
        Some(views::hint(&q, n, true))
    }

    pub(super) fn cmd_solution_session(&mut self) -> Option<Entry> {
        let q = self.current_question()?.clone();
        let lang = self.config.lang();
        let Some(a) = self.session.as_mut()?.attempt.as_mut() else {
            self.explained_now(q.meta.id);
            return Some(views::solution(&q, lang));
        };
        if !a.solution_armed {
            a.solution_armed = true;
            return Some(views::warn(
                "Viewing the solution marks this attempt as revealed.\n/solution again to confirm, or /hint for a smaller nudge.",
            ));
        }
        a.solution_viewed = true;
        self.explained_now(q.meta.id);
        self.record("solution_viewed", serde_json::Value::Null);
        self.save_attempt(None, None);
        Some(views::solution(&q, a_lang(&self.session).unwrap_or(lang)))
    }

    /// Moves on without solving. Counted as a fail when tests were run (it
    /// was tried), a skip otherwise. Doesn't jump ahead, so `/solution` can
    /// still show how it's done.
    pub(super) fn cmd_skip(&mut self) -> Option<Entry> {
        let tried = match self.attempt_or_err() {
            Ok(a) => a.test_runs,
            Err(e) => return Some(e),
        };
        self.finish(skip_outcome(tried), true);
        None
    }

    pub(super) fn cmd_pause(&mut self) -> Option<Entry> {
        let a = match self.session.as_mut().and_then(|s| s.attempt.as_mut()) {
            Some(a) => a,
            None => return self.attempt_or_err().err(),
        };
        if !a.timer.pause() {
            return Some(views::info("already paused  ·  /edit to get back to it"));
        }
        self.record("pause", serde_json::Value::Null);
        self.save_attempt(None, None);
        Some(views::info("⏸ paused  ·  /edit when you're back"))
    }

    pub(super) fn cmd_next(&mut self) -> Option<Entry> {
        let Some(s) = self.session.as_mut() else {
            return Some(views::error("no session  ·  /solve <id> to start"));
        };
        if s.attempt.is_some() {
            return Some(views::error("finish this one first  ·  /submit or /skip"));
        }
        if !s.has_next() {
            return Some(views::info(
                "that was the last question  ·  /solve for more, /quit for your summary",
            ));
        }
        s.index += 1;
        self.start_question(self.config.lang(), None)
    }

    /// Leaves the current attempt open to continue later: its time, counters
    /// and code are saved, and the working file stays on disk.
    fn suspend(&mut self) {
        let Some(a) = self.session.as_mut().and_then(|s| s.attempt.as_mut()) else {
            return;
        };
        a.timer.pause();
        let code = std::fs::read_to_string(&a.file).ok();
        self.record("suspend", serde_json::Value::Null);
        self.save_attempt(None, code.as_deref());
        let Some(s) = self.session.as_mut() else {
            return;
        };
        if let Some(a) = s.attempt.take() {
            s.done.push(
                Done::builder()
                    .question_id(a.question_id)
                    .outcome(Outcome::Unfinished)
                    .elapsed(a.timer.elapsed())
                    .hints(a.hints_used)
                    .runs(a.test_runs)
                    .failed_runs(a.failed_runs)
                    .build(),
            );
        }
    }

    /// Closes the session when dojo closes: pauses any open attempt and
    /// keeps the summary for printing after exit.
    pub(super) fn end_session(&mut self, show: bool) {
        self.cancel_tests();
        self.suspend();
        self.refresh_unfinished();
        let Some(s) = self.session.take() else { return };
        if let Err(e) = self.store.end_session(s.id) {
            self.notify(format!("could not close session: {e:#}"));
        }
        if show {
            self.transcript
                .push(views::session_summary(&self.bank, &s.done));
        }
        self.last_summary = Some(s.done);
    }
}

/// Cases a run covers, so a crashed run still reports e.g. 0/3.
fn q_case_count(bank: &crate::questions::Bank, id: u32, which: Which) -> usize {
    bank.get(id).map_or(0, |q| {
        q.cases
            .iter()
            .filter(|c| which == Which::All || !c.hidden)
            .count()
    })
}

fn a_lang(session: &Option<Session>) -> Option<Language> {
    Some(session.as_ref()?.attempt.as_ref()?.lang)
}

/// Up to `count` distinct ids from `pool`, in random order.
fn pick_random(mut pool: Vec<u32>, count: usize) -> Vec<u32> {
    // xorshift seeded from the clock; good enough for picking practice.
    let mut x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0x9e37_79b9, |d| d.as_nanos() as u64)
        | 1;
    let mut next = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    for i in (1..pool.len()).rev() {
        pool.swap(i, (next() % (i as u64 + 1)) as usize);
    }
    pool.truncate(count);
    pool
}

/// Not completed: a fail if tests were run (it was tried), a skip otherwise.
fn skip_outcome(test_runs: u32) -> Outcome {
    if test_runs > 0 {
        Outcome::Fail
    } else {
        Outcome::Skip
    }
}

/// `23 min`, `1 h 05 min`.
fn away_for(d: Duration) -> String {
    let m = d.as_secs() / 60;
    if m >= 60 {
        format!("{} h {:02} min", m / 60, m % 60)
    } else {
        format!("{m} min")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[rstest::rstest]
    #[case(0, "0 min")]
    #[case(23 * 60 + 59, "23 min")]
    #[case(60 * 60, "1 h 00 min")]
    #[case(125 * 60, "2 h 05 min")]
    fn away_for_reads_well(#[case] secs: u64, #[case] expected: &str) {
        assert_eq!(super::away_for(Duration::from_secs(secs)), expected);
    }

    #[test]
    fn picks_distinct_random_ids() {
        let picked = super::pick_random((1..=17).collect(), 5);
        assert_eq!(picked.len(), 5);
        let mut sorted = picked.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 5);
        assert!(picked.iter().all(|id| (1..=17).contains(id)));
        assert_eq!(super::pick_random(vec![1, 2], 9).len(), 2);
    }
}
