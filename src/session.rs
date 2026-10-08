//! Practice sessions: the question queue, the attempt in progress, its timer,
//! workspace files and editor launching. No UI here.

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use anyhow::{Context, Result, bail};

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

    /// Pauses as if it had been paused at `at` (when the user stepped
    /// away), so time since then isn't counted.
    pub fn pause_at(&mut self, at: Instant) -> bool {
        match self.since.take() {
            Some(s) => {
                self.banked += at.saturating_duration_since(s);
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

#[derive(Debug, bon::Builder)]
pub struct Attempt {
    pub id: i64,
    pub question_id: u32,
    pub lang: Language,
    pub dir: PathBuf,
    pub file: PathBuf,
    pub timer: Timer,
    #[builder(default)]
    pub test_runs: u32,
    #[builder(default)]
    pub failed_runs: u32,
    #[builder(default)]
    pub hints_used: usize,
    #[builder(default)]
    pub solution_viewed: bool,
    /// `/solution` was asked once; asking again reveals it.
    #[builder(default)]
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
#[derive(Debug, Clone, bon::Builder)]
pub struct Done {
    pub question_id: u32,
    pub outcome: Outcome,
    pub elapsed: Duration,
    pub hints: usize,
    pub runs: u32,
    pub failed_runs: u32,
}

#[derive(Debug, bon::Builder)]
pub struct Session {
    pub id: i64,
    /// How the current queue was chosen (`id`, `search`) and the query.
    pub mode: String,
    pub query: Option<String>,
    pub queue: Vec<u32>,
    /// Index into `queue` of the current (or next) question.
    #[builder(default)]
    pub index: usize,
    pub attempt: Option<Attempt>,
    #[builder(default)]
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
#[bon::builder]
pub fn prepare_work(
    root: &Path,
    question: &Question,
    lang: Language,
    saved: Option<&str>,
    fresh: bool,
) -> Result<(PathBuf, PathBuf, bool)> {
    let q = question;
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
#[bon::builder]
pub fn editor_command(template: &str, file: &Path, dir: &Path, line: usize) -> String {
    let mut t = template.trim().to_string();
    if !t.contains("{file}") && !t.contains("{dir}") {
        t.push_str(" {file}");
    }
    if !t.contains("{line}") && t.contains("{file}") {
        let program = t.split_whitespace().next().unwrap_or_default();
        t = match program_name(program).as_str() {
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

/// Editors that always run inside the terminal.
const TERMINAL_EDITORS: &[&str] = &[
    "vi",
    "vim",
    "nvim",
    "nvi",
    "vis",
    "hx",
    "helix",
    "nano",
    "pico",
    "micro",
    "kak",
    "joe",
    "jed",
    "ne",
    "mg",
    "zile",
    "dte",
    "mcedit",
    "ed",
    "emacs-nox",
    // Debian's pick of the system editor, usually nano or vim.
    "editor",
    "sensible-editor",
];

/// VS Code and its forks: their CLI hands the file to a window, even from a
/// remote terminal with no display of its own.
const CODE_EDITORS: &[&str] = &["code", "code-insiders", "code-oss", "codium", "cursor"];

/// An editor program's name: no directory, lowercase, without Debian's
/// alternative suffixes (`vim.basic`, `vim.tiny`, `vim.nox`) or `.appimage`.
fn program_name(program: &str) -> String {
    let name = Path::new(program)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(program)
        .to_ascii_lowercase();
    [".basic", ".tiny", ".nox", ".appimage"]
        .iter()
        .find_map(|suffix| name.strip_suffix(suffix))
        .map_or_else(|| name.clone(), str::to_string)
}

/// Where `program` is: itself when it's a path, else the first match on PATH.
fn find_program(program: &str) -> Option<PathBuf> {
    if program.contains('/') {
        let path = PathBuf::from(program);
        return path.is_file().then_some(path);
    }
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(program))
        .find(|path| path.is_file())
}

/// Whether GUI editors have somewhere to open: always on macOS, elsewhere
/// only with an X11 or Wayland display.
fn has_display() -> bool {
    cfg!(target_os = "macos")
        || std::env::var_os("DISPLAY").is_some()
        || std::env::var_os("WAYLAND_DISPLAY").is_some()
}

/// Whether an editor command takes over the terminal (vs. opening a window).
pub fn is_terminal_editor(command: &str) -> bool {
    let resolve = |program: &str| find_program(program).and_then(|p| std::fs::canonicalize(p).ok());
    classify_editor(command, has_display(), &resolve)
}

/// `is_terminal_editor` with the display check and symlink resolution
/// passed in.
fn classify_editor(
    command: &str,
    display: bool,
    resolve: &dyn Fn(&str) -> Option<PathBuf>,
) -> bool {
    let mut words = command.split_whitespace();
    let program = words.next().unwrap_or_default();
    let args: Vec<&str> = words.collect();
    // The name as typed and the program it links to (Debian's `editor` →
    // /etc/alternatives/editor → vim.basic). Either can be the telling one:
    // `vi` may link to busybox.
    let mut names = vec![program_name(program)];
    if let Some(real) = resolve(program) {
        names.push(program_name(&real.to_string_lossy()));
    }
    let named = |list: &[&str]| names.iter().any(|n| list.contains(&n.as_str()));
    if named(CODE_EDITORS) {
        false
    } else if !display || named(TERMINAL_EDITORS) {
        // With no display, anything else can only run in the terminal.
        true
    } else if named(&["emacs", "emacsclient"]) {
        args.iter()
            .any(|a| matches!(*a, "-nw" | "-t" | "--tty" | "-tty" | "--no-window-system"))
    } else {
        false
    }
}

/// How long a GUI editor gets to fail before it counts as started.
const GUI_START: Duration = Duration::from_millis(300);

/// Opens a GUI editor without waiting for it. Its stderr goes to `log`, so a
/// failure to start can point there.
pub fn launch_gui(command: &str, log: &Path) -> Result<()> {
    let program = command.split_whitespace().next().unwrap_or_default();
    // Only a plain program name can be checked (not `FOO=1 x` or `"$EDITOR"`).
    let plain = !program.is_empty() && !program.contains(['=', '\'', '"', '$', '~', '\\']);
    if plain && find_program(program).is_none() {
        bail!("`{program}` not found on your PATH");
    }
    if let Some(dir) = log.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let stderr = std::fs::File::create(log).map_or_else(|_| Stdio::null(), Stdio::from);
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(stderr);
    // Its own process group, so Ctrl-C or closing the terminal doesn't take
    // the editor with it.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd.spawn().context("could not start the editor")?;
    // An editor that can't start exits at once; one that started keeps
    // running or hands the file to a running instance and exits 0.
    let started = Instant::now();
    while started.elapsed() < GUI_START {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            }
            if status.code() == Some(127) {
                bail!("`{program}` not found (see {})", log.display());
            }
            bail!("{program} exited with an error (see {})", log.display());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    // Reap it in the background so it never lingers as a zombie.
    std::thread::spawn(move || child.wait());
    Ok(())
}

/// Runs a command in the foreground (a terminal editor, an installer). The
/// caller must have handed over the terminal first.
pub fn launch_terminal(command: &str) -> Result<bool> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .context("could not start sh")?;
    // Like system(3): Ctrl-C and Ctrl-\ are for the child while it runs.
    // Ignored only after spawning, so the child doesn't inherit SIG_IGN.
    #[cfg(unix)]
    let _interrupts = IgnoreInterrupts::new();
    let status = child.wait().context("waiting for the command")?;
    Ok(status.success())
}

/// Ignores SIGINT and SIGQUIT in dojo until dropped.
#[cfg(unix)]
struct IgnoreInterrupts {
    int: libc::sighandler_t,
    quit: libc::sighandler_t,
}

#[cfg(unix)]
impl IgnoreInterrupts {
    fn new() -> IgnoreInterrupts {
        // SAFETY: SIG_IGN is a valid disposition, and the previous ones are
        // put back on drop.
        unsafe {
            IgnoreInterrupts {
                int: libc::signal(libc::SIGINT, libc::SIG_IGN),
                quit: libc::signal(libc::SIGQUIT, libc::SIG_IGN),
            }
        }
    }
}

#[cfg(unix)]
impl Drop for IgnoreInterrupts {
    fn drop(&mut self) {
        // SAFETY: restores the dispositions `signal` returned.
        unsafe {
            libc::signal(libc::SIGINT, self.int);
            libc::signal(libc::SIGQUIT, self.quit);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn pausing_at_a_past_moment_drops_the_time_since() {
        let start = Instant::now() - Duration::from_secs(600);
        let mut t = Timer {
            banked: Duration::from_secs(30),
            since: Some(start),
        };
        // Last seen 4 minutes in; the 6 minutes since aren't counted.
        assert!(t.pause_at(start + Duration::from_secs(240)));
        assert!(t.paused());
        assert_eq!(t.elapsed(), Duration::from_secs(270));
        assert!(!t.pause_at(Instant::now()), "already paused");
    }

    #[test]
    fn pausing_before_the_timer_started_counts_nothing() {
        let mut t = Timer::resume_from(Duration::from_secs(5));
        t.pause_at(Instant::now() - Duration::from_secs(60));
        assert_eq!(t.elapsed(), Duration::from_secs(5));
    }

    #[rstest]
    #[case("nvim {file}", true, None, true)]
    #[case("/usr/bin/vim", true, None, true)]
    #[case("vi", true, None, true)]
    #[case("hx {file}:{line}", true, None, true)]
    #[case("nano", true, None, true)]
    #[case("kak", true, None, true)]
    #[case("vis", true, None, true)]
    #[case("dte", true, None, true)]
    #[case("mcedit", true, None, true)]
    #[case("emacs -nw {file}", true, None, true)]
    #[case("emacsclient -t", true, None, true)]
    #[case("emacs {file}", true, None, false)]
    #[case("code {file}", true, None, false)]
    #[case("zed", true, None, false)]
    #[case("subl -w", true, None, false)]
    #[case::debian_alternative("editor", true, Some("/usr/bin/vim.basic"), true)]
    #[case::unresolved_alternative("editor", true, None, true)]
    #[case("sensible-editor", true, None, true)]
    #[case("vim.tiny {file}", true, None, true)]
    #[case("/opt/nvim.appimage", true, None, true)]
    #[case("/opt/Nvim.AppImage", true, None, true)]
    #[case::busybox("vi", true, Some("/bin/busybox"), true)]
    #[case::nox_emacs("emacs", true, Some("/usr/bin/emacs-nox"), true)]
    #[case::gui_link("myedit", true, Some("/usr/bin/gedit"), false)]
    #[case::no_display_emacs("emacs {file}", false, None, true)]
    #[case::no_display_gedit("gedit", false, None, true)]
    #[case::no_display_code("code {file}", false, None, false)]
    #[case::no_display_cursor("cursor", false, None, false)]
    #[case::no_display_codium("codium", false, None, false)]
    #[case::no_display_code_oss("/usr/bin/code-oss", false, None, false)]
    fn classifies_editors(
        #[case] command: &str,
        #[case] display: bool,
        #[case] links_to: Option<&str>,
        #[case] terminal: bool,
    ) {
        let resolve = |_: &str| links_to.map(PathBuf::from);
        assert_eq!(classify_editor(command, display, &resolve), terminal);
    }

    #[rstest]
    #[case("vim.basic", "vim")]
    #[case("/usr/bin/vim.nox", "vim")]
    #[case("Cursor.AppImage", "cursor")]
    #[case("nvim", "nvim")]
    #[case("emacs-nox", "emacs-nox")]
    fn normalizes_program_names(#[case] program: &str, #[case] expected: &str) {
        assert_eq!(program_name(program), expected);
    }

    #[rstest]
    #[case::started("true", None)]
    #[case::still_running("sleep 1", None)]
    #[case::missing(
        "dojo-no-such-editor {file}",
        Some("`dojo-no-such-editor` not found on your PATH")
    )]
    #[case::exit_127("sh -c 'exit 127'", Some("`sh` not found (see"))]
    #[case::failed("false", Some("false exited with an error (see"))]
    fn reports_gui_editors_that_fail(#[case] command: &str, #[case] error: Option<&str>) {
        let tmp = tempfile::tempdir().unwrap();
        let log = tmp.path().join("state/editor.log");
        match (launch_gui(command, &log), error) {
            (Ok(()), None) => {}
            (Err(e), Some(start)) => assert!(e.to_string().starts_with(start), "{e}"),
            (got, want) => panic!("{command}: got {got:?}, want {want:?}"),
        }
    }

    #[rstest]
    #[case(
        "code {dir} {file}",
        "/w/it's/solution.py",
        3,
        r"code '/w/it'\''s' -g '/w/it'\''s/solution.py':3"
    )]
    #[case("nvim", "/w/it's/solution.py", 7, r"nvim +7 '/w/it'\''s/solution.py'")]
    #[case("nvim {file}", "/w/s.py", 7, "nvim +7 '/w/s.py'")]
    #[case("vim", "/w/s.py", 1, "vim +1 '/w/s.py'")]
    #[case("emacs -nw {file}", "/w/s.py", 9, "emacs -nw +9 '/w/s.py'")]
    #[case("hx {file}", "/w/s.py", 2, "hx '/w/s.py':2")]
    #[case("zed", "/w/s.py", 5, "zed '/w/s.py':5")]
    #[case("cursor {file}", "/w/s.py", 5, "cursor -g '/w/s.py':5")]
    #[case(
        "myedit --line {line} {file}",
        "/w/s.py",
        4,
        "myedit --line 4 '/w/s.py'"
    )]
    #[case("gedit", "/w/s.py", 4, "gedit '/w/s.py'")]
    #[case("nvim", "/w/s.py", 0, "nvim +1 '/w/s.py'")]
    #[case("vim.basic", "/w/s.py", 3, "vim.basic +3 '/w/s.py'")]
    fn expands_editor_templates(
        #[case] template: &str,
        #[case] file: &str,
        #[case] line: usize,
        #[case] expected: &str,
    ) {
        let file = Path::new(file);
        let command = editor_command()
            .template(template)
            .file(file)
            .dir(file.parent().unwrap())
            .line(line)
            .call();
        assert_eq!(command, expected);
    }

    const PY: &str = "# 1. Two Sum\n\ndef two_sum(nums, target):\n    pass\n";
    const JS: &str = "// x\n\nfunction twoSum(nums, target) {\n\n}\n";

    #[rstest]
    #[case::untouched_python(PY, PY, 4)]
    #[case::untouched_javascript(JS, JS, 4)]
    #[case::edited_body(
        PY,
        "# 1. Two Sum\n\ndef two_sum(nums, target):\n    seen = {}\n    for i, x in enumerate(nums):\n        pass\n",
        6
    )]
    #[case::helper_on_top(
        PY,
        "import heapq\n# 1. Two Sum\n\ndef two_sum(nums, target):\n    pass\n",
        1
    )]
    #[case::appended_helper(
        PY,
        "# 1. Two Sum\n\ndef two_sum(nums, target):\n    pass\n\ndef helper():\n    return 1\n",
        7
    )]
    #[case::emptied(PY, "", 1)]
    fn finds_resume_line(#[case] boilerplate: &str, #[case] code: &str, #[case] line: usize) {
        assert_eq!(resume_line(boilerplate, code), line);
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
    fn timer_resumes_from_banked_time() {
        let t = Timer::resume_from(Duration::from_secs(310));
        assert!(t.elapsed() >= Duration::from_secs(310));
        assert!(!t.paused());
    }

    #[rstest]
    #[case(0, "00:00")]
    #[case(61, "01:01")]
    #[case(599, "09:59")]
    #[case(3599, "59:59")]
    #[case(3600, "1:00:00")]
    #[case(3725, "1:02:05")]
    fn formats_clock(#[case] secs: u64, #[case] expected: &str) {
        assert_eq!(clock(Duration::from_secs(secs)), expected);
    }
}
