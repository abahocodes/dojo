//! Runs a solution against a question's test cases.
//!
//! Each language has a small harness (`harness.py`, `harness.js`) speaking
//! the same JSON protocol: it loads the solution, decodes inputs, calls the
//! function per case with a timeout and reports raw return values. Comparison
//! against expected output happens here, so every language is judged the same
//! way.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use serde_json::{Value, json};

pub use crate::lang::Language;

pub mod compiled;
use crate::questions::{Compare, Question};

/// Per-case time limit inside the harness.
pub const CASE_TIMEOUT: Duration = Duration::from_secs(3);

const PY_HARNESS: &str = include_str!("harness.py");
const JS_HARNESS: &str = include_str!("harness.js");

/// What runs a language: the program (overridable with an environment
/// variable), how to ask its version, the oldest version dojo supports and
/// how to install it.
struct Tool {
    var: &'static str,
    program: &'static str,
    version_args: &'static [&'static str],
    label: &'static str,
    min: (u32, u32),
    /// How the minimum is shown: `3.10+`, `18+`.
    min_shown: &'static str,
    install: &'static str,
}

fn tool(lang: Language) -> Tool {
    match lang {
        Language::Python => Tool {
            var: "DOJO_PYTHON",
            program: "python3",
            version_args: &["--version"],
            label: "Python",
            min: (3, 10),
            min_shown: "3.10+",
            install: "install Python 3.10+",
        },
        Language::JavaScript => Tool {
            var: "DOJO_NODE",
            program: "node",
            version_args: &["--version"],
            label: "Node.js",
            min: (18, 0),
            min_shown: "18+",
            install: "install Node.js 18+",
        },
        // Node runs TypeScript itself (type stripping, 22.13+).
        Language::TypeScript => Tool {
            var: "DOJO_NODE",
            program: "node",
            version_args: &["--version"],
            label: "Node.js",
            min: (22, 13),
            min_shown: "22.13+",
            install: "install Node.js 22.13+ (it runs TypeScript directly)",
        },
        Language::Java => Tool {
            var: "DOJO_JAVAC",
            program: "javac",
            version_args: &["-version"],
            label: "Java",
            min: (17, 0),
            min_shown: "17+",
            install: "install a JDK 17+ (e.g. brew install openjdk, or apt install openjdk-17-jdk)",
        },
        Language::Cpp => Tool {
            var: "DOJO_CXX",
            program: "c++",
            version_args: &["--version"],
            label: "C++ compiler",
            min: (7, 0),
            min_shown: "C++17 support (GCC 7+ or Clang 5+)",
            install: "install a C++17 compiler (Xcode command line tools, or apt install g++)",
        },
        Language::Go => Tool {
            var: "DOJO_GO",
            program: "go",
            version_args: &["version"],
            label: "Go",
            min: (1, 21),
            min_shown: "1.21+",
            install: "install Go 1.21+ (https://go.dev/dl)",
        },
    }
}

/// The program for a language, overridable with e.g. `DOJO_PYTHON`.
fn program(lang: Language) -> String {
    let t = tool(lang);
    std::env::var(t.var).unwrap_or_else(|_| t.program.to_string())
}

fn install_hint(lang: Language) -> &'static str {
    tool(lang).install
}

/// Checks the language's toolchain is installed and new enough, returning
/// its version.
pub fn toolchain(lang: Language) -> Result<String> {
    let program = program(lang);
    let out = Command::new(&program)
        .args(tool(lang).version_args)
        .output()
        .map_err(|e| anyhow!("{program} not found ({e}); {}", install_hint(lang)))?;
    // Python 2 and javac print their version to stderr.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let text = if stdout.trim().is_empty() {
        stderr
    } else {
        stdout
    };
    let first = text.lines().next().unwrap_or_default();
    check_version(lang, first.trim())
}

/// `major.minor.patch` from `--version` output (`Python 3.12.1`,
/// `v18.19.0`, `Python 3.13.0rc1`, `go version go1.23.2 darwin/arm64`,
/// `javac 21.0.2`).
fn parse_version(text: &str) -> Option<(u32, u32, u32)> {
    let word = text
        .split_whitespace()
        .map(|w| w.strip_prefix("go").unwrap_or(w))
        .map(|w| w.strip_prefix('v').unwrap_or(w))
        .find(|w| w.starts_with(|c: char| c.is_ascii_digit()))?;
    let mut parts = word.split('.').map(|p| {
        let digits: String = p.chars().take_while(char::is_ascii_digit).collect();
        digits.parse::<u32>().ok()
    });
    let major = parts.next()??;
    let minor = parts.next().flatten().unwrap_or(0);
    let patch = parts.next().flatten().unwrap_or(0);
    Some((major, minor, patch))
}

/// The version to show, or an error when it's older than dojo supports.
/// Output that can't be parsed is let through.
fn check_version(lang: Language, text: &str) -> Result<String> {
    let t = tool(lang);
    let Some((major, minor, patch)) = parse_version(text) else {
        return Ok(format!("{} {text}", t.label));
    };
    let shown = format!("{} {major}.{minor}.{patch}", t.label);
    if (major, minor) < t.min {
        bail!("{shown} is too old; dojo needs {}", t.min_shown);
    }
    Ok(shown)
}

/// Which test cases to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// Visible cases only (`/test`).
    Visible,
    /// Every case including hidden ones (`/submit`).
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Fail,
    Error,
    Timeout,
    /// Skipped because an earlier case timed out or crashed the run.
    NotRun,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct CaseResult {
    pub index: usize,
    pub hidden: bool,
    pub status: Status,
    pub got: Option<Value>,
    pub error: Option<String>,
    pub stdout: Option<String>,
    pub ms: f64,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct RunReport {
    pub which: Which,
    pub cases: Vec<CaseResult>,
    /// Set when the solution could not run at all (syntax error, missing
    /// function, whole-run timeout).
    pub fatal: Option<String>,
    /// Output printed while the file was loaded (top-level prints).
    pub load_stdout: Option<String>,
    pub elapsed: Duration,
}

impl RunReport {
    pub fn passed(&self) -> usize {
        self.cases
            .iter()
            .filter(|c| c.status == Status::Pass)
            .count()
    }

    pub fn total(&self) -> usize {
        self.cases.len()
    }

    pub fn all_passed(&self) -> bool {
        self.fatal.is_none() && !self.cases.is_empty() && self.passed() == self.total()
    }
}

/// Raw per-case result as written by a harness.
#[derive(Debug, Deserialize, bon::Builder)]
struct RawResult {
    index: usize,
    status: String,
    #[serde(default)]
    got: Option<Value>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    stdout: Option<String>,
    #[serde(default)]
    #[builder(default)]
    ms: f64,
}

#[derive(Debug, Deserialize, bon::Builder)]
struct RawResults {
    #[serde(default)]
    fatal: Option<String>,
    #[serde(default)]
    load_stdout: Option<String>,
    #[serde(default)]
    #[builder(default)]
    results: Vec<RawResult>,
}

/// Runs `solution` against `question`'s cases. A set `cancel` flag stops
/// the run as soon as it's noticed.
#[bon::builder]
pub fn run(
    question: &Question,
    lang: Language,
    solution: &Path,
    which: Which,
    cancel: Option<&AtomicBool>,
) -> Result<RunReport> {
    let never = AtomicBool::new(false);
    let cancel = cancel.unwrap_or(&never);
    let q = question;
    let indices: Vec<usize> = q
        .cases
        .iter()
        .enumerate()
        .filter(|(_, c)| which == Which::All || !c.hidden)
        .map(|(i, _)| i)
        .collect();
    if indices.is_empty() {
        bail!("question {} has no test cases to run", q.id());
    }

    let start = Instant::now();
    let raw = run_harness()
        .question(q)
        .lang(lang)
        .solution(solution)
        .indices(&indices)
        .cancel(cancel)
        .call()?;
    let elapsed = start.elapsed();

    let cases = raw
        .results
        .into_iter()
        .map(|r| {
            let case = &q.cases[r.index];
            let status = match r.status.as_str() {
                "ok" => match &r.got {
                    Some(got) if matches(q.meta.compare, &case.output, got) => Status::Pass,
                    _ => Status::Fail,
                },
                "timeout" => Status::Timeout,
                "not_run" => Status::NotRun,
                _ => Status::Error,
            };
            CaseResult::builder()
                .index(r.index)
                .hidden(case.hidden)
                .status(status)
                .maybe_got(r.got)
                .maybe_error(r.error)
                .maybe_stdout(r.stdout)
                .ms(r.ms)
                .build()
        })
        .collect();

    Ok(RunReport::builder()
        .which(which)
        .cases(cases)
        .maybe_fatal(raw.fatal)
        .maybe_load_stdout(raw.load_stdout.filter(|s| !s.trim().is_empty()))
        .elapsed(elapsed)
        .build())
}

#[bon::builder]
fn run_harness(
    question: &Question,
    lang: Language,
    solution: &Path,
    indices: &[usize],
    cancel: &AtomicBool,
) -> Result<RawResults> {
    let q = question;
    let dir = tempfile::tempdir()?;
    let spec_path = dir.path().join("spec.json");
    let results_path = dir.path().join("results.json");
    let progress_path = dir.path().join("results.json.progress");

    let sig = &q.meta.signature;
    let spec = json!({
        "function": lang.function_name(&sig.function),
        "params": sig.params.iter().map(|p| json!({"name": p.name, "type": p.ty.to_string()})).collect::<Vec<_>>(),
        "returns": sig.returns.to_string(),
        "timeout_secs": CASE_TIMEOUT.as_secs_f64(),
        "cases": indices.iter().map(|&i| json!({"index": i, "input": q.cases[i].input})).collect::<Vec<_>>(),
    });
    std::fs::write(&spec_path, serde_json::to_vec(&spec)?)?;
    let solution = solution
        .canonicalize()
        .with_context(|| format!("solution not found: {}", solution.display()))?;

    let mut cmd = if lang.compiled() {
        match compiled::build()
            .lang(lang)
            .signature(sig)
            .solution(&solution)
            .dir(dir.path())
            .cancel(cancel)
            .call()?
        {
            compiled::Built::Ready(cmd) => cmd,
            compiled::Built::Failed(fatal) => {
                return Ok(RawResults::builder().fatal(fatal).build());
            }
        }
    } else {
        interpreted(lang, dir.path(), &solution)?
    };
    cmd.arg(&spec_path).arg(&results_path);

    let program = program(lang);
    let limit = CASE_TIMEOUT * indices.len() as u32 + Duration::from_secs(5);
    let out = exec()
        .cmd(&mut cmd)
        .limit(limit)
        .cancel(cancel)
        .progress(&progress_path)
        .call()
        .map_err(|e| anyhow!("could not start {program} ({e}); {}", install_hint(lang)))?;

    let step = std::fs::read_to_string(&progress_path).unwrap_or_default();
    let step = step.trim();
    let partial = || -> Option<RawResults> {
        std::fs::read(&results_path)
            .ok()
            .and_then(|d| serde_json::from_slice(&d).ok())
    };
    // Stopped from outside (a step that stalled, e.g. a loop inside C code
    // that can't be interrupted) or crashed: report which step, keeping the
    // results of the cases that finished.
    let stop = if out.stalled || out.timed_out {
        Some(Stop::Timeout)
    } else if let Some(sig) = out.signal {
        Some(Stop::Crash(sig))
    } else if !out.success && !step.is_empty() && step != "done" {
        // Died mid-run with an error of its own (a Go stack overflow, an
        // uncaught C++ exception): its last words explain why.
        Some(Stop::Died(died_reason(&format!(
            "{}{}",
            out.stdout, out.stderr
        ))))
    } else {
        None
    };
    if let Some(reason) = stop {
        return Ok(stopped_results()
            .step(step)
            .maybe_partial(partial())
            .indices(indices)
            .reason(reason)
            .lang(lang)
            .call());
    }
    match std::fs::read(&results_path) {
        Ok(data) => Ok(serde_json::from_slice(&data).context("harness wrote invalid results")?),
        Err(_) if !out.success => Ok(RawResults::builder()
            .fatal(format!("{}{}", out.stdout, out.stderr).trim().to_string())
            .build()),
        Err(e) => bail!("harness produced no results: {e}"),
    }
}

/// The command for an interpreted language's harness, before its spec and
/// results arguments.
fn interpreted(lang: Language, dir: &Path, solution: &Path) -> Result<Command> {
    let (harness_name, harness_src) = match lang {
        Language::Python => ("harness.py", PY_HARNESS),
        _ => ("harness.js", JS_HARNESS),
    };
    let harness = dir.join(harness_name);
    std::fs::write(&harness, harness_src)?;
    let mut cmd = Command::new(program(lang));
    match lang {
        Language::Python => {
            cmd.args(["-X", "utf8"]).env("PYTHONDONTWRITEBYTECODE", "1");
        }
        _ => {
            // Runaway allocations fail fast instead of swapping.
            cmd.env("NODE_OPTIONS", "").arg("--max-old-space-size=1024");
        }
    }
    cmd.arg(&harness).arg(solution);
    Ok(cmd)
}

/// Why a program died, from its output: the runtime's own reason lines
/// (Go's `fatal error:`, an uncaught C++ exception, a Java exception) when
/// there are any, else its last lines.
fn died_reason(text: &str) -> String {
    const REASONS: &[&str] = &[
        "runtime: goroutine stack exceeds",
        "fatal error:",
        "panic:",
        "terminate called",
        "  what():",
        "Exception in thread",
        "Caused by:",
    ];
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let reasons: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| REASONS.iter().any(|r| l.starts_with(r)))
        .take(4)
        .collect();
    if reasons.is_empty() {
        lines[lines.len().saturating_sub(12)..].join("\n")
    } else {
        reasons.join("\n")
    }
}

/// Runs a Python snippet that prints one JSON document, with a time limit.
/// Used for question authoring (stress-case generators), never for user code.
pub fn python_json(code: &str, limit: Duration) -> Result<Value> {
    let dir = tempfile::tempdir()?;
    let script = dir.path().join("generate.py");
    std::fs::write(&script, code)?;
    let program = program(Language::Python);
    let out = exec()
        .cmd(Command::new(&program).args(["-X", "utf8"]).arg(&script))
        .limit(limit)
        .cancel(&AtomicBool::new(false))
        .call()
        .map_err(|e| anyhow!("could not start {program} ({e})"))?;
    if out.timed_out {
        bail!("took longer than {}s", limit.as_secs());
    }
    if !out.success {
        bail!("{}", out.stderr.trim());
    }
    serde_json::from_str(out.stdout.trim()).context("did not print valid JSON")
}

/// Why a run was stopped before the harness finished.
enum Stop {
    Timeout,
    /// Killed by this signal: a segfault from runaway recursion, or the OS
    /// killing it for memory.
    Crash(i32),
    /// Exited with an error before finishing; its last output.
    Died(String),
}

/// Signal numbers differ by OS (SIGBUS is 10 on macOS, 7 on Linux, where 10
/// is SIGUSR1), so they come from libc.
#[cfg(unix)]
fn signal_name(sig: i32) -> &'static str {
    match sig {
        libc::SIGABRT => "aborted",
        libc::SIGKILL => "killed, often for using too much memory",
        libc::SIGBUS => "bus error",
        libc::SIGSEGV => "segmentation fault",
        _ => "terminated",
    }
}

#[cfg(not(unix))]
fn signal_name(_sig: i32) -> &'static str {
    "terminated"
}

/// Builds results for a run stopped at `step` (`load`, a case index, or
/// `done`): finished cases keep their results, the step that hung or
/// crashed is reported, and the rest are marked not run.
#[bon::builder]
fn stopped_results(
    step: &str,
    partial: Option<RawResults>,
    indices: &[usize],
    reason: Stop,
    lang: Language,
) -> RawResults {
    let load_stdout = partial.as_ref().and_then(|p| p.load_stdout.clone());
    let current = match step.parse::<usize>() {
        Ok(i) => i,
        Err(_) if step == "done" => {
            if let Some(p) = partial {
                return p;
            }
            usize::MAX
        }
        Err(_) => {
            // Never got past loading the file.
            let fatal = match reason {
                Stop::Timeout => format!(
                    "loading your file took over {}s and was stopped: is there a loop at the top level of the file?",
                    CASE_TIMEOUT.as_secs()
                ),
                Stop::Crash(sig) => format!(
                    "the {} process crashed while loading your file ({})",
                    lang.label(),
                    signal_name(sig)
                ),
                Stop::Died(ref why) => format!("your program stopped while starting:\n{why}"),
            };
            return RawResults::builder()
                .fatal(fatal)
                .maybe_load_stdout(load_stdout)
                .build();
        }
    };
    let mut results: Vec<RawResult> = partial
        .map(|p| p.results)
        .unwrap_or_default()
        .into_iter()
        .filter(|r| r.index != current)
        .collect();
    let done: std::collections::BTreeSet<usize> = results.iter().map(|r| r.index).collect();
    let mut reached = false;
    for &i in indices {
        if done.contains(&i) {
            continue;
        }
        let blank = |status: &str, error: Option<String>| {
            RawResult::builder()
                .index(i)
                .status(status.into())
                .maybe_error(error)
                .build()
        };
        if i == current {
            reached = true;
            results.push(match reason {
                Stop::Timeout => blank("timeout", None),
                Stop::Died(ref why) => blank(
                    "error",
                    Some(format!("your program stopped on this case:\n{why}")),
                ),
                Stop::Crash(sig) => blank(
                    "error",
                    Some(format!(
                        "the {} process crashed on this case ({})\nhint: usually runaway recursion or running out of memory",
                        lang.label(),
                        signal_name(sig)
                    )),
                ),
            });
        } else if reached {
            results.push(blank("not_run", None));
        }
    }
    results.sort_by_key(|r| indices.iter().position(|i| *i == r.index));
    RawResults::builder()
        .maybe_load_stdout(load_stdout)
        .results(results)
        .build()
}

/// Output of a finished child process.
#[derive(bon::Builder)]
struct ExecOutput {
    success: bool,
    stdout: String,
    stderr: String,
    timed_out: bool,
    /// Killed because the harness's progress file stopped changing.
    stalled: bool,
    /// The signal that killed it, when it didn't exit normally.
    signal: Option<i32>,
}

/// How long one step (loading the file, or one case) may run before dojo
/// stops it from outside. The harness enforces `CASE_TIMEOUT` itself; this
/// catches what it can't interrupt.
const STALL: Duration = Duration::from_secs(CASE_TIMEOUT.as_secs() + 2);

/// Most of a child's own output dojo keeps (crash messages); the rest is
/// drained and dropped.
const PIPE_CAP: u64 = 64 * 1024;

/// Runs a command with a wall-clock limit, capturing output without risking a
/// full-pipe deadlock.
#[bon::builder]
fn exec(
    cmd: &mut Command,
    limit: Duration,
    cancel: &AtomicBool,
    progress: Option<&Path>,
) -> Result<ExecOutput> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let capped = |mut pipe: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut kept = Vec::new();
            let _ = (&mut pipe).take(PIPE_CAP).read_to_end(&mut kept);
            let _ = std::io::copy(&mut pipe, &mut std::io::sink());
            String::from_utf8_lossy(&kept).into_owned()
        })
    };
    let out_t = capped(Box::new(child.stdout.take().context("no stdout")?));
    let err_t = capped(Box::new(child.stderr.take().context("no stderr")?));

    let deadline = Instant::now() + limit;
    let mut last_step = String::new();
    let mut step_since = Instant::now();
    let mut last_check = Instant::now();
    let (status, timed_out, stalled) = loop {
        if let Some(status) = child.try_wait()? {
            break (Some(status), false, false);
        }
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("cancelled");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break (None, true, false);
        }
        if let Some(path) = progress
            && last_check.elapsed() >= Duration::from_millis(100)
        {
            last_check = Instant::now();
            let step = std::fs::read_to_string(path).unwrap_or_default();
            if step != last_step {
                last_step = step;
                step_since = Instant::now();
            } else if !last_step.is_empty() && last_step != "done" && step_since.elapsed() > STALL {
                let _ = child.kill();
                let _ = child.wait();
                break (None, false, true);
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    };

    #[cfg(unix)]
    let signal = {
        use std::os::unix::process::ExitStatusExt;
        status.and_then(|s| s.signal())
    };
    #[cfg(not(unix))]
    let signal = None;

    Ok(ExecOutput::builder()
        .success(status.is_some_and(|s| s.success()))
        .stdout(out_t.join().unwrap_or_default())
        .stderr(err_t.join().unwrap_or_default())
        .timed_out(timed_out)
        .stalled(stalled)
        .maybe_signal(signal)
        .build())
}

/// Compares a returned value with the expected one under a compare mode.
pub fn matches(mode: Compare, expected: &Value, got: &Value) -> bool {
    match mode {
        Compare::Exact => expected == got,
        Compare::Float => float_eq(expected, got),
        Compare::Unordered => match (expected, got) {
            (Value::Array(a), Value::Array(b)) => sorted(a) == sorted(b),
            _ => expected == got,
        },
        Compare::UnorderedDeep => canonical(expected) == canonical(got),
    }
}

fn float_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (
                x.as_f64().unwrap_or(f64::NAN),
                y.as_f64().unwrap_or(f64::NAN),
            );
            (x - y).abs() <= 1e-6 * x.abs().max(1.0)
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| float_eq(a, b))
        }
        _ => a == b,
    }
}

fn sorted(items: &[Value]) -> Vec<String> {
    let mut keys: Vec<String> = items.iter().map(Value::to_string).collect();
    keys.sort();
    keys
}

fn canonical(v: &Value) -> Value {
    match v {
        Value::Array(items) => {
            let mut items: Vec<Value> = items.iter().map(canonical).collect();
            items.sort_by_key(Value::to_string);
            Value::Array(items)
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod runaway_tests {
    //! Solutions that loop forever, recurse forever, crash or flood output
    //! must be stopped quickly and reported clearly, in every language.

    use rstest::rstest;

    use super::*;
    use crate::questions::Bank;

    fn run_code(lang: Language, code: &str) -> (RunReport, Duration) {
        let q = Bank::embedded().get(1).unwrap().clone(); // two_sum, 3 visible
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join(lang.solution_file());
        std::fs::write(&file, code).unwrap();
        let start = Instant::now();
        let report = run()
            .question(&q)
            .lang(lang)
            .solution(&file)
            .which(Which::Visible)
            .call()
            .unwrap();
        (report, start.elapsed())
    }

    fn statuses(r: &RunReport) -> Vec<Status> {
        r.cases.iter().map(|c| c.status).collect()
    }

    #[rstest]
    #[case::python_loop(
        Language::Python,
        "def two_sum(nums, target):\n    while True:\n        print('spin')\n"
    )]
    #[case::javascript_loop(
        Language::JavaScript,
        "function twoSum(nums, target) { for (;;) { console.log('spin'); } }\n"
    )]
    #[case::python_uninterruptible(
        Language::Python,
        "def two_sum(nums, target):\n    return sum(range(10**15))\n"
    )]
    fn runaway_loop_stops_at_the_first_timeout(#[case] lang: Language, #[case] code: &str) {
        let (r, took) = run_code(lang, code);
        assert_eq!(
            statuses(&r),
            [Status::Timeout, Status::NotRun, Status::NotRun]
        );
        assert!(took < Duration::from_secs(10), "took {took:?}");
        // A print flood is capped as it happens.
        let out = r.cases[0].stdout.as_deref().unwrap_or_default();
        assert!(out.len() < 5_000, "{} bytes kept", out.len());
    }

    #[rstest]
    #[case::python(
        Language::Python,
        "def two_sum(nums, target):\n    return two_sum(nums, target)\n",
        "RecursionError"
    )]
    #[case::javascript(
        Language::JavaScript,
        "function twoSum(nums, target) { return twoSum(nums, target); }\n",
        "Maximum call stack"
    )]
    fn infinite_recursion_is_explained(
        #[case] lang: Language,
        #[case] code: &str,
        #[case] error: &str,
    ) {
        let (r, _) = run_code(lang, code);
        assert_eq!(r.cases[0].status, Status::Error);
        let err = r.cases[0].error.as_deref().unwrap_or_default();
        assert!(
            err.contains(error) && err.contains("hint: recursion"),
            "{err}"
        );
        assert!(err.lines().count() < 40, "traceback not collapsed:\n{err}");
    }

    #[rstest]
    #[case::python(Language::Python, "while True:\n    pass\n")]
    #[case::javascript(Language::JavaScript, "for (;;) {}\n")]
    fn top_level_loop_is_caught_while_loading(#[case] lang: Language, #[case] code: &str) {
        let (r, took) = run_code(lang, code);
        let fatal = r.fatal.unwrap_or_default();
        assert!(fatal.contains("top level"), "{fatal}");
        assert!(took < Duration::from_secs(10), "took {took:?}");
    }

    #[rstest]
    #[case::python(
        Language::Python,
        "def two_sum(nums, target):\n    print('looking at', nums)\n    return [0, 0]\n",
        "looking at"
    )]
    #[case::javascript(
        Language::JavaScript,
        "function twoSum(nums, target) { console.log('looking at', nums); return [0, 0]; }\n",
        "looking at"
    )]
    fn prints_are_captured_per_case(
        #[case] lang: Language,
        #[case] code: &str,
        #[case] printed: &str,
    ) {
        let (r, _) = run_code(lang, code);
        assert!(
            r.cases
                .iter()
                .all(|c| c.stdout.as_deref().is_some_and(|o| o.contains(printed)))
        );
    }

    #[test]
    fn crash_is_reported_on_its_case() {
        // The process kills itself the way the OS kills a process that runs
        // out of memory. (A real segfault would also work, but macOS shows a
        // "Python quit unexpectedly" dialog for every one.)
        let (r, _) = run_code(
            Language::Python,
            "import os, signal\n\ndef two_sum(nums, target):\n    if len(nums) == 2:\n        os.kill(os.getpid(), signal.SIGKILL)\n    return []\n",
        );
        // Case 1 (4 numbers) finished; case 2 ([5, 5]) crashed; case 3 not run.
        assert_eq!(statuses(&r), [Status::Fail, Status::Error, Status::NotRun]);
        let err = r.cases[1].error.as_deref().unwrap_or_default();
        assert!(err.contains("crashed") && err.contains("memory"), "{err}");
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::{Value, json};

    use super::*;

    #[rstest]
    #[case(Compare::Exact, json!([1, 2]), json!([1, 2]), true)]
    #[case(Compare::Exact, json!([1, 2]), json!([2, 1]), false)]
    #[case(Compare::Exact, json!(3), json!(3.0), false)]
    #[case(Compare::Exact, json!(null), json!(null), true)]
    #[case(Compare::Unordered, json!([1, 2]), json!([2, 1]), true)]
    #[case(Compare::Unordered, json!([1, 1, 2]), json!([1, 2, 2]), false)]
    #[case(Compare::Unordered, json!([[1, 2]]), json!([[2, 1]]), false)]
    #[case(Compare::Unordered, json!(5), json!(5), true)]
    #[case(Compare::UnorderedDeep, json!([[1, 2], [3]]), json!([[3], [2, 1]]), true)]
    #[case(Compare::UnorderedDeep, json!([[1, 2], [3]]), json!([[3], [2, 2]]), false)]
    #[case(Compare::Float, json!(0.1), json!(0.1000000001), true)]
    #[case(Compare::Float, json!(0.1), json!(0.2), false)]
    #[case(Compare::Float, json!([1.0, 2.5]), json!([1, 2.5000001]), true)]
    #[case(Compare::Float, json!(1e9), json!(1e9 + 1.0), true)]
    fn compares(
        #[case] mode: Compare,
        #[case] expected: Value,
        #[case] got: Value,
        #[case] same: bool,
    ) {
        assert_eq!(matches(mode, &expected, &got), same);
    }

    #[rstest]
    #[case("Python 3.12.1", Some((3, 12, 1)))]
    #[case("Python 3.13.0rc1", Some((3, 13, 0)))]
    #[case("v18.19.0", Some((18, 19, 0)))]
    #[case("v22.1", Some((22, 1, 0)))]
    #[case("Python 2.7.18", Some((2, 7, 18)))]
    #[case("", None)]
    #[case("not a version", None)]
    fn parses_versions(#[case] text: &str, #[case] expected: Option<(u32, u32, u32)>) {
        assert_eq!(parse_version(text), expected);
    }

    #[rstest]
    #[case::go_stack_overflow(
        "runtime: goroutine stack exceeds 1000000000-byte limit\nruntime: sp=0x1 stack=[0x1, 0x2]\nfatal error: stack overflow\n\nruntime stack:\nruntime.throw(...)\n",
        "runtime: goroutine stack exceeds 1000000000-byte limit\nfatal error: stack overflow"
    )]
    #[case::cpp_exception(
        "terminate called after throwing an instance of 'std::out_of_range'\n  what():  vector::_M_range_check\n",
        "terminate called after throwing an instance of 'std::out_of_range'\n  what():  vector::_M_range_check"
    )]
    #[case::no_reason("a\n\nb\nc\n", "a\nb\nc")]
    fn picks_why_a_program_died(#[case] output: &str, #[case] expected: &str) {
        assert_eq!(died_reason(output), expected);
    }

    #[rstest]
    #[case(Language::Python, "Python 3.12.1", Ok("Python 3.12.1"))]
    #[case(Language::Python, "Python 3.10.0", Ok("Python 3.10.0"))]
    #[case(
        Language::Python,
        "Python 3.9.2",
        Err("Python 3.9.2 is too old; dojo needs 3.10+")
    )]
    #[case(
        Language::Python,
        "Python 2.7.18",
        Err("Python 2.7.18 is too old; dojo needs 3.10+")
    )]
    #[case(Language::JavaScript, "v18.0.0", Ok("Node.js 18.0.0"))]
    #[case(Language::JavaScript, "v22.11.0", Ok("Node.js 22.11.0"))]
    #[case(
        Language::JavaScript,
        "v16.20.2",
        Err("Node.js 16.20.2 is too old; dojo needs 18+")
    )]
    #[case(Language::JavaScript, "weird", Ok("Node.js weird"))]
    #[case(Language::TypeScript, "v22.13.0", Ok("Node.js 22.13.0"))]
    #[case(
        Language::TypeScript,
        "v22.11.0",
        Err("Node.js 22.11.0 is too old; dojo needs 22.13+")
    )]
    #[case(Language::Go, "go version go1.23.2 darwin/arm64", Ok("Go 1.23.2"))]
    #[case(
        Language::Go,
        "go version go1.19 linux/amd64",
        Err("Go 1.19.0 is too old; dojo needs 1.21+")
    )]
    #[case(Language::Java, "javac 21.0.2", Ok("Java 21.0.2"))]
    #[case(
        Language::Java,
        "javac 11.0.20",
        Err("Java 11.0.20 is too old; dojo needs 17+")
    )]
    #[case(
        Language::Cpp,
        "Apple clang version 21.0.0 (clang-2100.3.34.2)",
        Ok("C++ compiler 21.0.0")
    )]
    #[case(
        Language::Cpp,
        "g++ (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0",
        Ok("C++ compiler 11.4.0")
    )]
    fn checks_versions(
        #[case] lang: Language,
        #[case] text: &str,
        #[case] expected: Result<&str, &str>,
    ) {
        let got = check_version(lang, text).map_err(|e| e.to_string());
        assert_eq!(
            got.as_deref(),
            expected.map_err(|e| e.to_string()).as_deref()
        );
    }
}

/// The runner works for every language: values of every type make it into
/// solutions and back out, right answers pass, wrong ones fail, and hidden
/// cases run only on submit. Built on small fixture questions (not the
/// bank), so this suite grows with languages, not with questions.
#[cfg(test)]
mod language_tests {
    use std::collections::BTreeMap;

    use rstest::rstest;
    use serde_json::{Value, json};

    use super::*;

    struct Fixture {
        question: Question,
        code: &'static [(Language, &'static str)],
    }

    impl Fixture {
        fn code(&self, lang: Language) -> &'static str {
            self.code
                .iter()
                .find(|(l, _)| *l == lang)
                .map(|(_, c)| *c)
                .unwrap_or_else(|| panic!("fixture has no {lang} code"))
        }
    }

    fn question(
        function: &str,
        params: Value,
        returns: &str,
        compare: &str,
        cases: Value,
    ) -> Question {
        let meta = json!({
            "id": 9000, "slug": "runner-fixture", "title": "Runner fixture",
            "difficulty": "easy", "tags": ["fixture"], "target_minutes": 1,
            "signature": { "function": function, "params": params, "returns": returns },
            "compare": compare,
            "statement": "statement.md", "hints": "hints.md",
            "explanation": "explanation.md", "tests": "tests.json",
            "languages": {
                "python": { "boilerplate": "b.py", "solution": "s.py" },
                "javascript": { "boilerplate": "b.js", "solution": "s.js" }
            }
        });
        Question::builder()
            .dir("9000-runner-fixture".into())
            .meta(serde_json::from_value(meta).unwrap())
            .statement(String::new())
            .hints(vec![])
            .explanation(String::new())
            .cases(serde_json::from_value(cases).unwrap())
            .boilerplate(BTreeMap::new())
            .solutions(BTreeMap::new())
            .unreferenced(vec![])
            .build()
    }

    fn fixture(name: &str) -> Fixture {
        match name {
            // int[] and int in, int[] out
            "ints" => Fixture {
                question: question(
                    "scale",
                    json!([{ "name": "values", "type": "int[]" }, { "name": "factor", "type": "int" }]),
                    "int[]",
                    "exact",
                    json!([
                        { "input": { "values": [1, 2, 3], "factor": 2 }, "output": [2, 4, 6] },
                        { "input": { "values": [], "factor": 5 }, "output": [] },
                        { "input": { "values": [-1, 0, 7], "factor": -3 }, "output": [3, 0, -21], "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def scale(values, factor):\n    return [v * factor for v in values]\n",
                    ),
                    (
                        Language::JavaScript,
                        "function scale(values, factor) { return values.map((v) => v * factor); }\n",
                    ),
                    (
                        Language::TypeScript,
                        "function scale(values: number[], factor: number): number[] { return values.map((v) => v * factor); }\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc scale(values []int, factor int) []int {\n\tout := make([]int, len(values))\n\tfor i, v := range values {\n\t\tout[i] = v * factor\n\t}\n\treturn out\n}\n",
                    ),
                ],
            },
            // nested lists
            "nested" => Fixture {
                question: question(
                    "transpose",
                    json!([{ "name": "grid", "type": "int[][]" }]),
                    "int[][]",
                    "exact",
                    json!([
                        { "input": { "grid": [[1, 2], [3, 4]] }, "output": [[1, 3], [2, 4]] },
                        { "input": { "grid": [[1, 2, 3]] }, "output": [[1], [2], [3]] },
                        { "input": { "grid": [] }, "output": [], "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def transpose(grid):\n    return [list(r) for r in zip(*grid)]\n",
                    ),
                    (
                        Language::JavaScript,
                        "function transpose(grid) {\n  return grid.length ? grid[0].map((_, c) => grid.map((r) => r[c])) : [];\n}\n",
                    ),
                    (
                        Language::TypeScript,
                        "function transpose(grid: number[][]): number[][] {\n  return grid.length ? grid[0].map((_, c) => grid.map((r) => r[c])) : [];\n}\n",
                    ),
                    // Returns nil for an empty grid: Go's idiomatic empty slice.
                    (
                        Language::Go,
                        "package main\n\nfunc transpose(grid [][]int) [][]int {\n\tif len(grid) == 0 {\n\t\treturn nil\n\t}\n\tout := make([][]int, len(grid[0]))\n\tfor c := range out {\n\t\tout[c] = make([]int, len(grid))\n\t\tfor r := range grid {\n\t\t\tout[c][r] = grid[r][c]\n\t\t}\n\t}\n\treturn out\n}\n",
                    ),
                ],
            },
            "bool" => Fixture {
                question: question(
                    "negate",
                    json!([{ "name": "flag", "type": "bool" }]),
                    "bool",
                    "exact",
                    json!([
                        { "input": { "flag": true }, "output": false },
                        { "input": { "flag": false }, "output": true }
                    ]),
                ),
                code: &[
                    (Language::Python, "def negate(flag):\n    return not flag\n"),
                    (
                        Language::JavaScript,
                        "function negate(flag) { return !flag; }\n",
                    ),
                    (
                        Language::TypeScript,
                        "function negate(flag: boolean): boolean { return !flag; }\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc negate(flag bool) bool { return !flag }\n",
                    ),
                ],
            },
            "float" => Fixture {
                question: question(
                    "halve",
                    json!([{ "name": "x", "type": "float" }]),
                    "float",
                    "float",
                    json!([
                        { "input": { "x": 1 }, "output": 0.5 },
                        { "input": { "x": 0.3 }, "output": 0.15 },
                        { "input": { "x": -7.5 }, "output": -3.75, "hidden": true }
                    ]),
                ),
                code: &[
                    (Language::Python, "def halve(x):\n    return x / 2\n"),
                    (
                        Language::JavaScript,
                        "function halve(x) { return x / 2; }\n",
                    ),
                    (
                        Language::TypeScript,
                        "function halve(x: number): number { return x / 2; }\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc halve(x float64) float64 { return x / 2 }\n",
                    ),
                ],
            },
            "strings" => Fixture {
                question: question(
                    "lengths",
                    json!([{ "name": "words", "type": "string[]" }]),
                    "int[]",
                    "exact",
                    json!([
                        { "input": { "words": ["a", "bb", ""] }, "output": [1, 2, 0] },
                        { "input": { "words": [] }, "output": [] },
                        { "input": { "words": ["道場", "dojo"] }, "output": [2, 4], "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def lengths(words):\n    return [len(w) for w in words]\n",
                    ),
                    (
                        Language::JavaScript,
                        "function lengths(words) { return words.map((w) => w.length); }\n",
                    ),
                    (
                        Language::TypeScript,
                        "function lengths(words: string[]): number[] { return words.map((w) => [...w].length); }\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nimport \"unicode/utf8\"\n\nfunc lengths(words []string) []int {\n\tout := []int{}\n\tfor _, w := range words {\n\t\tout = append(out, utf8.RuneCountInString(w))\n\t}\n\treturn out\n}\n",
                    ),
                ],
            },
            "list_node" => Fixture {
                question: question(
                    "reverse",
                    json!([{ "name": "head", "type": "ListNode" }]),
                    "ListNode",
                    "exact",
                    json!([
                        { "input": { "head": [1, 2, 3] }, "output": [3, 2, 1] },
                        { "input": { "head": [] }, "output": [] },
                        { "input": { "head": [7] }, "output": [7], "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def reverse(head):\n    prev = None\n    while head:\n        head.next, prev, head = prev, head, head.next\n    return prev\n",
                    ),
                    (
                        Language::JavaScript,
                        "function reverse(head) {\n  let prev = null;\n  while (head) { const next = head.next; head.next = prev; prev = head; head = next; }\n  return prev;\n}\n",
                    ),
                    (
                        Language::TypeScript,
                        "function reverse(head: ListNode | null): ListNode | null {\n  let prev: ListNode | null = null;\n  while (head) { const next = head.next; head.next = prev; prev = head; head = next; }\n  return prev;\n}\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc reverse(head *ListNode) *ListNode {\n\tvar prev *ListNode\n\tfor head != nil {\n\t\thead.Next, prev, head = prev, head, head.Next\n\t}\n\treturn prev\n}\n",
                    ),
                ],
            },
            "tree_node" => Fixture {
                question: question(
                    "mirror",
                    json!([{ "name": "root", "type": "TreeNode" }]),
                    "TreeNode",
                    "exact",
                    json!([
                        { "input": { "root": [1, 2, 3] }, "output": [1, 3, 2] },
                        { "input": { "root": [1, null, 2] }, "output": [1, 2] },
                        { "input": { "root": [] }, "output": [] },
                        { "input": { "root": [4, 2, 7, 1, 3, 6, 9] }, "output": [4, 7, 2, 9, 6, 3, 1], "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def mirror(root):\n    if root:\n        root.left, root.right = mirror(root.right), mirror(root.left)\n    return root\n",
                    ),
                    (
                        Language::JavaScript,
                        "function mirror(root) {\n  if (root) { [root.left, root.right] = [mirror(root.right), mirror(root.left)]; }\n  return root;\n}\n",
                    ),
                    (
                        Language::TypeScript,
                        "function mirror(root: TreeNode | null): TreeNode | null {\n  if (root) { [root.left, root.right] = [mirror(root.right), mirror(root.left)]; }\n  return root;\n}\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc mirror(root *TreeNode) *TreeNode {\n\tif root != nil {\n\t\troot.Left, root.Right = mirror(root.Right), mirror(root.Left)\n\t}\n\treturn root\n}\n",
                    ),
                ],
            },
            // 64-bit values (Java long, C++ long long)
            "long" => Fixture {
                question: question(
                    "total",
                    json!([{ "name": "values", "type": "long[]" }]),
                    "long",
                    "exact",
                    json!([
                        { "input": { "values": [4000000000_i64, 5000000000_i64] }, "output": 9000000000_i64 },
                        { "input": { "values": [] }, "output": 0 },
                        { "input": { "values": [4503599627370496_i64, 4503599627370495_i64] }, "output": 9007199254740991_i64, "hidden": true }
                    ]),
                ),
                code: &[
                    (
                        Language::Python,
                        "def total(values):\n    return sum(values)\n",
                    ),
                    (
                        Language::JavaScript,
                        "function total(values) { return values.reduce((a, b) => a + b, 0); }\n",
                    ),
                    (
                        Language::TypeScript,
                        "function total(values: number[]): number { return values.reduce((a, b) => a + b, 0); }\n",
                    ),
                    (
                        Language::Go,
                        "package main\n\nfunc total(values []int) int {\n\tsum := 0\n\tfor _, v := range values {\n\t\tsum += v\n\t}\n\treturn sum\n}\n",
                    ),
                ],
            },
            other => panic!("no fixture {other}"),
        }
    }

    fn run_fixture(f: &Fixture, lang: Language, code: &str, which: Which) -> RunReport {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join(lang.solution_file());
        std::fs::write(&file, code).unwrap();
        run()
            .question(&f.question)
            .lang(lang)
            .solution(&file)
            .which(which)
            .call()
            .unwrap()
    }

    /// Every type round-trips and correct solutions pass every case.
    #[rstest]
    fn correct_solutions_pass(
        #[values(
            Language::Python,
            Language::JavaScript,
            Language::TypeScript,
            Language::Go
        )]
        lang: Language,
        #[values(
            "ints",
            "nested",
            "bool",
            "float",
            "strings",
            "list_node",
            "tree_node",
            "long"
        )]
        name: &str,
    ) {
        let f = fixture(name);
        let report = run_fixture(&f, lang, f.code(lang), Which::All);
        assert!(report.fatal.is_none(), "{:?}", report.fatal);
        let failed: Vec<_> = report
            .cases
            .iter()
            .filter(|c| c.status != Status::Pass)
            .map(|c| (c.index, c.status, c.got.clone(), c.error.clone()))
            .collect();
        assert!(failed.is_empty(), "{lang} {name}: {failed:?}");
        assert_eq!(report.total(), f.question.cases.len());
    }

    /// Wrong answers fail and report what the solution returned, converted
    /// back from the language's own types.
    #[rstest]
    #[case::python(Language::Python, "def reverse(head):\n    return head\n")]
    #[case::javascript(Language::JavaScript, "function reverse(head) { return head; }\n")]
    #[case::typescript(
        Language::TypeScript,
        "function reverse(head: ListNode | null): ListNode | null { return head; }\n"
    )]
    #[case::go(
        Language::Go,
        "package main\n\nfunc reverse(head *ListNode) *ListNode { return head }\n"
    )]
    fn wrong_answers_fail_with_what_they_returned(#[case] lang: Language, #[case] code: &str) {
        let f = fixture("list_node");
        let report = run_fixture(&f, lang, code, Which::Visible);
        assert_eq!(report.cases[0].status, Status::Fail);
        assert_eq!(report.cases[0].got, Some(json!([1, 2, 3])));
        assert_eq!(report.cases[1].status, Status::Pass); // [] reversed is []
    }

    /// `/test` runs visible cases; `/submit` adds the hidden ones.
    #[rstest]
    fn hidden_cases_run_only_on_submit(
        #[values(
            Language::Python,
            Language::JavaScript,
            Language::TypeScript,
            Language::Go
        )]
        lang: Language,
    ) {
        let f = fixture("tree_node");
        let visible = run_fixture(&f, lang, f.code(lang), Which::Visible);
        let all = run_fixture(&f, lang, f.code(lang), Which::All);
        assert_eq!(visible.total(), 3);
        assert_eq!(all.total(), 4);
        assert!(all.cases.iter().any(|c| c.hidden));
    }

    /// A missing function is reported, not a crash.
    #[rstest]
    #[case::python(Language::Python, "def something_else():\n    pass\n")]
    #[case::javascript(Language::JavaScript, "function somethingElse() {}\n")]
    #[case::typescript(Language::TypeScript, "function somethingElse(): void {}\n")]
    #[case::go(Language::Go, "package main\n\nfunc somethingElse() {}\n")]
    fn missing_function_is_reported(#[case] lang: Language, #[case] code: &str) {
        let f = fixture("bool");
        let report = run_fixture(&f, lang, code, Which::Visible);
        let fatal = report.fatal.unwrap_or_default();
        assert!(
            fatal.contains("`negate` not found") || fatal.contains("couldn't call `negate`"),
            "{fatal}"
        );
    }

    /// Compile errors point at the solution's own lines.
    #[test]
    fn go_compile_errors_are_reported() {
        let f = fixture("bool");
        let code = "package main\n\nfunc negate(flag bool) bool {\n\treturn flag +\n}\n";
        let fatal = run_fixture(&f, Language::Go, code, Which::Visible)
            .fatal
            .unwrap_or_default();
        assert!(fatal.starts_with("compile error\nsolution.go:"), "{fatal}");
    }

    /// A panic fails its case with the message and the solution's frames;
    /// the other cases still run.
    #[test]
    fn go_panics_fail_their_case() {
        let f = fixture("ints");
        let code = "package main\n\nfunc scale(values []int, factor int) []int {\n\tif factor == 5 {\n\t\t_ = values[10]\n\t}\n\treturn values\n}\n";
        let report = run_fixture(&f, Language::Go, code, Which::Visible);
        assert_eq!(report.cases[0].status, Status::Fail);
        assert_eq!(report.cases[1].status, Status::Error);
        let error = report.cases[1].error.clone().unwrap_or_default();
        assert!(error.contains("index out of range"), "{error}");
        assert!(error.contains("solution.go:5"), "{error}");
    }

    /// A stack overflow can't be recovered in Go: the run stops on that case
    /// with Go's own reason.
    #[test]
    fn go_stack_overflow_is_reported_on_its_case() {
        let f = fixture("bool");
        let code = "package main\n\nfunc negate(flag bool) bool {\n\treturn negate(flag)\n}\n";
        let report = run_fixture(&f, Language::Go, code, Which::Visible);
        assert_eq!(report.cases[0].status, Status::Error);
        let error = report.cases[0].error.clone().unwrap_or_default();
        assert!(error.contains("stack overflow"), "{error}");
        assert_eq!(report.cases[1].status, Status::NotRun);
    }

    /// Prints are captured per case.
    #[test]
    fn go_prints_are_captured() {
        let f = fixture("bool");
        let code = "package main\n\nimport \"fmt\"\n\nfunc negate(flag bool) bool {\n\tfmt.Println(\"seen\", flag)\n\treturn !flag\n}\n";
        let report = run_fixture(&f, Language::Go, code, Which::Visible);
        assert_eq!(report.cases[0].stdout.as_deref(), Some("seen true"));
        assert_eq!(report.cases[1].stdout.as_deref(), Some("seen false"));
    }
}
