//! Practice sessions: the question queue, the attempt in progress, its timer,
//! workspace files and editor launching. No UI here.

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use anyhow::{Context, Result};

use crate::lang::Language;
use crate::questions::Question;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Passed every test without viewing the solution.
    Pass,
    /// Passed after viewing the solution.
    Revealed,
    /// Skipped after running tests: tried, not solved.
    Fail,
    /// Skipped without running tests.
    Skip,
    /// Left open to continue later. Shown in summaries; never stored (an open
    /// attempt has no outcome).
    Unfinished,
}

impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Pass => "pass",
            Outcome::Revealed => "revealed",
            Outcome::Fail => "fail",
            Outcome::Skip => "skip",
            Outcome::Unfinished => "unfinished",
        }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Wall-clock timer that can be paused.
#[derive(Debug, Clone)]
pub struct Timer {
    banked: Duration,
    since: Option<Instant>,
}

impl Timer {
    pub fn start() -> Timer {
        Timer::resume_from(Duration::ZERO)
    }

    /// A running timer that already has `banked` on the clock, for
    /// continuing an attempt.
    pub fn resume_from(banked: Duration) -> Timer {
        Timer {
            banked,
            since: Some(Instant::now()),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.banked + self.since.map_or(Duration::ZERO, |s| s.elapsed())
    }

    pub fn paused(&self) -> bool {
        self.since.is_none()
    }

    pub fn pause(&mut self) -> bool {
        match self.since.take() {
            Some(s) => {
                self.banked += s.elapsed();
                true
            }
            None => false,
        }
    }

    pub fn resume(&mut self) -> bool {
        if self.since.is_none() {
            self.since = Some(Instant::now());
            true
        } else {
            false
        }
    }
}

pub fn clock(d: Duration) -> String {
    let s = d.as_secs();
    if s >= 3600 {
        format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60)
    } else {
        format!("{:02}:{:02}", s / 60, s % 60)
    }
}

#[derive(Debug)]
pub struct Attempt {
    pub id: i64,
    pub question_id: u32,
    pub lang: Language,
    pub dir: PathBuf,
    pub file: PathBuf,
    pub timer: Timer,
    pub test_runs: u32,
    pub failed_runs: u32,
    pub hints_used: usize,
    pub solution_viewed: bool,
    /// `/solution` was asked once; asking again reveals it.
    pub solution_armed: bool,
    /// (passed, total) of the latest run.
    pub last_run: Option<(usize, usize)>,
    /// Solution file mtime when last checked, for re-running tests on save.
    pub mtime: Option<SystemTime>,
}

impl Attempt {
    pub fn file_mtime(&self) -> Option<SystemTime> {
        std::fs::metadata(&self.file)
            .and_then(|m| m.modified())
            .ok()
    }
}

/// A finished attempt, kept for the end-of-session summary.
#[derive(Debug, Clone)]
pub struct Done {
    pub question_id: u32,
    pub outcome: Outcome,
    pub elapsed: Duration,
    pub hints: usize,
    pub runs: u32,
    pub failed_runs: u32,
}

#[derive(Debug)]
pub struct Session {
    pub id: i64,
    /// How the current queue was chosen (`id`, `search`) and the query.
    pub mode: String,
    pub query: Option<String>,
    pub queue: Vec<u32>,
    /// Index into `queue` of the current (or next) question.
    pub index: usize,
    pub attempt: Option<Attempt>,
    pub done: Vec<Done>,
}

impl Session {
    pub fn has_next(&self) -> bool {
        self.index + 1 < self.queue.len()
    }
}

/// The scratch file an attempt is edited in:
/// `<root>/<NNNN-slug>/<lang>/solution.<ext>`. Sessions are not folders; they
/// live in the database.
pub fn work_paths(root: &Path, q: &Question, lang: Language) -> (PathBuf, PathBuf) {
    let dir = root.join(&q.dir).join(lang.name());
    let file = dir.join(lang.solution_file());
    (dir, file)
}

/// Prepares the working file. Code already on disk is kept unless `fresh`;
/// otherwise it's written from `saved` (code recorded for an open attempt)
/// or the question's boilerplate. Returns `true` when existing code was kept.
pub fn prepare_work(
    root: &Path,
    q: &Question,
    lang: Language,
    saved: Option<&str>,
    fresh: bool,
) -> Result<(PathBuf, PathBuf, bool)> {
    let boilerplate = q
        .boilerplate
        .get(&lang)
        .with_context(|| format!("#{} has no {} boilerplate", q.meta.id, lang.label()))?;
    let (dir, file) = work_paths(root, q, lang);
    if !fresh
        && let Ok(existing) = std::fs::read_to_string(&file)
        && existing.trim() != boilerplate.trim()
    {
        return Ok((dir, file, true));
    }
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let code = if fresh { None } else { saved };
    std::fs::write(&file, code.unwrap_or(boilerplate))?;
    Ok((dir, file, code.is_some()))
}

/// Removes a finished attempt's working folder; its code is in the database.
pub fn clear_work(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
    if let Some(parent) = dir.parent() {
        let _ = std::fs::remove_dir(parent); // only succeeds when empty
    }
}

fn quote(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', r"'\''"))
}

/// Builds the shell command that opens `file` at `line`.
///
/// `{file}`, `{dir}` and `{line}` are substituted; without `{file}` or
/// `{dir}` the file is appended. When the template has no `{line}`, the line
/// is added for editors that take one (`nvim +12 f`, `code -g f:12`,
/// `hx f:12`); other editors just open the file.
pub fn editor_command(template: &str, file: &Path, dir: &Path, line: usize) -> String {
    let mut t = template.trim().to_string();
    if !t.contains("{file}") && !t.contains("{dir}") {
        t.push_str(" {file}");
    }
    if !t.contains("{line}") && t.contains("{file}") {
        let program = t.split_whitespace().next().unwrap_or_default();
        let name = Path::new(program)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(program);
        t = match name {
            // `+N file`
            "vi" | "vim" | "nvim" | "nano" | "emacs" | "emacsclient" | "kak" | "mg" => {
                t.replacen("{file}", "+{line} {file}", 1)
            }
            // `-g file:N`
            "code" | "code-insiders" | "cursor" | "codium" | "windsurf" => {
                t.replacen("{file}", "-g {file}:{line}", 1)
            }
            // `file:N`
            "hx" | "helix" | "zed" | "subl" | "micro" => t.replacen("{file}", "{file}:{line}", 1),
            _ => t,
        };
    }
    t.replace("{file}", &quote(file))
        .replace("{dir}", &quote(dir))
        .replace("{line}", &line.max(1).to_string())
}

/// The 1-based line to put the cursor on: the last line changed from the
/// boilerplate, or the boilerplate's empty body (`pass` / blank line) when
/// nothing has been written yet.
pub fn resume_line(boilerplate: &str, code: &str) -> usize {
    let before: Vec<&str> = boilerplate.lines().collect();
    let after: Vec<&str> = code.lines().collect();
    let prefix = before
        .iter()
        .zip(&after)
        .take_while(|(a, b)| a.trim_end() == b.trim_end())
        .count();
    if prefix == before.len() && prefix == after.len() {
        // Untouched: the body placeholder, else the end.
        return after
            .iter()
            .rposition(|l| matches!(l.trim(), "" | "pass"))
            .filter(|&i| i > 0)
            .map_or(after.len().max(1), |i| i + 1);
    }
    let suffix = before[prefix..]
        .iter()
        .rev()
        .zip(after[prefix..].iter().rev())
        .take_while(|(a, b)| a.trim_end() == b.trim_end())
        .count();
    // Last line of the changed region in the user's code.
    (after.len() - suffix).max(prefix + 1)
}

/// Whether an editor command takes over the terminal (vs. opening a window).
pub fn is_terminal_editor(command: &str) -> bool {
    let mut words = command.split_whitespace();
    let program = words.next().unwrap_or_default();
    let name = Path::new(program)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(program);
    let args: Vec<&str> = words.collect();
    match name {
        "vi" | "vim" | "nvim" | "hx" | "helix" | "nano" | "micro" | "kak" | "joe" | "ne" | "mg"
        | "ed" | "pico" => true,
        "emacs" | "emacsclient" => args
            .iter()
            .any(|a| matches!(*a, "-nw" | "-t" | "--tty" | "-tty" | "--no-window-system")),
        _ => false,
    }
}

/// Opens a GUI editor without waiting for it.
pub fn launch_gui(command: &str) -> Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("could not start the editor")?;
    // Reap it in the background so it never lingers as a zombie.
    std::thread::spawn(move || child.wait());
    Ok(())
}

/// Runs a terminal editor in the foreground. The caller must have handed
/// over the terminal first.
pub fn launch_terminal(command: &str) -> Result<bool> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .context("could not start the editor")?;
    Ok(status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_editors() {
        assert!(is_terminal_editor("nvim {file}"));
        assert!(is_terminal_editor("/usr/bin/vim"));
        assert!(is_terminal_editor("emacs -nw {file}"));
        assert!(!is_terminal_editor("emacs {file}"));
        assert!(!is_terminal_editor("code {file}"));
        assert!(!is_terminal_editor("zed"));
    }

    #[test]
    fn expands_editor_templates() {
        let f = Path::new("/w/it's/solution.py");
        let d = Path::new("/w/it's");
        assert_eq!(
            editor_command("code {dir} {file}", f, d, 3),
            r"code '/w/it'\''s' -g '/w/it'\''s/solution.py':3"
        );
        assert_eq!(
            editor_command("nvim", f, d, 7),
            r"nvim +7 '/w/it'\''s/solution.py'"
        );
        let g = Path::new("/w/s.py");
        assert_eq!(editor_command("nvim {file}", g, d, 7), "nvim +7 '/w/s.py'");
        assert_eq!(editor_command("hx {file}", g, d, 2), "hx '/w/s.py':2");
        assert_eq!(
            editor_command("myedit --line {line} {file}", g, d, 4),
            "myedit --line 4 '/w/s.py'"
        );
        assert_eq!(editor_command("gedit", g, d, 4), "gedit '/w/s.py'");
    }

    #[test]
    fn finds_resume_line() {
        let py = "# 1. Two Sum\n\ndef two_sum(nums, target):\n    pass\n";
        assert_eq!(resume_line(py, py), 4); // the `pass`
        let js = "// x\n\nfunction twoSum(nums, target) {\n\n}\n";
        assert_eq!(resume_line(js, js), 4); // the empty body
        let edited = "# 1. Two Sum\n\ndef two_sum(nums, target):\n    seen = {}\n    for i, x in enumerate(nums):\n        pass\n";
        assert_eq!(resume_line(py, edited), 6);
        let helper_on_top = "import heapq\n# 1. Two Sum\n\ndef two_sum(nums, target):\n    pass\n";
        assert_eq!(resume_line(py, helper_on_top), 1);
    }

    #[test]
    fn timer_pauses() {
        let mut t = Timer::start();
        assert!(t.pause());
        let a = t.elapsed();
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(t.elapsed(), a);
        assert!(!t.pause());
        assert!(t.resume());
        assert!(!t.paused());
    }

    #[test]
    fn formats_clock() {
        assert_eq!(clock(Duration::from_secs(61)), "01:01");
        assert_eq!(clock(Duration::from_secs(3725)), "1:02:05");
    }
}
