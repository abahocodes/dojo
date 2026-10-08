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
use crate::questions::{Compare, Question};

/// Per-case time limit inside the harness.
pub const CASE_TIMEOUT: Duration = Duration::from_secs(3);

const PY_HARNESS: &str = include_str!("harness.py");
const JS_HARNESS: &str = include_str!("harness.js");

/// The interpreter for a language, overridable with `DOJO_PYTHON` /
/// `DOJO_NODE`.
fn interpreter(lang: Language) -> String {
    let (var, default) = match lang {
        Language::Python => ("DOJO_PYTHON", "python3"),
        Language::JavaScript => ("DOJO_NODE", "node"),
    };
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}

fn install_hint(lang: Language) -> &'static str {
    match lang {
        Language::Python => "install Python 3.10+",
        Language::JavaScript => "install Node.js 18+",
    }
}

/// Checks the language's toolchain is installed, returning its version.
pub fn toolchain(lang: Language) -> Result<String> {
    let program = interpreter(lang);
    let out = Command::new(&program)
        .arg("--version")
        .output()
        .map_err(|e| anyhow!("{program} not found ({e}); {}", install_hint(lang)))?;
    let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
    Ok(match lang {
        Language::JavaScript => format!("Node.js {version}"),
        Language::Python => version,
    })
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
    let (harness_name, harness_src) = match lang {
        Language::Python => ("harness.py", PY_HARNESS),
        Language::JavaScript => ("harness.js", JS_HARNESS),
    };
    let harness = dir.path().join(harness_name);
    let spec_path = dir.path().join("spec.json");
    let results_path = dir.path().join("results.json");

    let sig = &q.meta.signature;
    let spec = json!({
        "function": lang.function_name(&sig.function),
        "params": sig.params.iter().map(|p| json!({"name": p.name, "type": p.ty.to_string()})).collect::<Vec<_>>(),
        "returns": sig.returns.to_string(),
        "timeout_secs": CASE_TIMEOUT.as_secs_f64(),
        "cases": indices.iter().map(|&i| json!({"index": i, "input": q.cases[i].input})).collect::<Vec<_>>(),
    });
    std::fs::write(&harness, harness_src)?;
    std::fs::write(&spec_path, serde_json::to_vec(&spec)?)?;

    let solution = solution
        .canonicalize()
        .with_context(|| format!("solution not found: {}", solution.display()))?;
    let program = interpreter(lang);
    let mut cmd = Command::new(&program);
    match lang {
        Language::Python => {
            cmd.args(["-X", "utf8"]).env("PYTHONDONTWRITEBYTECODE", "1");
        }
        Language::JavaScript => {
            // Runaway allocations fail fast instead of swapping.
            cmd.env("NODE_OPTIONS", "").arg("--max-old-space-size=1024");
        }
    }
    cmd.arg(&harness)
        .arg(&solution)
        .arg(&spec_path)
        .arg(&results_path);

    let limit = CASE_TIMEOUT * indices.len() as u32 + Duration::from_secs(5);
    let progress_path = dir.path().join("results.json.progress");
    let out = exec()
        .cmd(&mut cmd)
        .limit(limit)
        .cancel(cancel)
        .progress(&progress_path)
        .call()
        .map_err(|e| anyhow!("could not start {program} ({e}); {}", install_hint(lang)))?;

    // Stopped from outside (a step that stalled, e.g. a loop inside C code
    // that can't be interrupted) or crashed: report which step, keeping the
    // results of the cases that finished.
    if out.stalled || out.timed_out || out.signal.is_some() {
        let step = std::fs::read_to_string(&progress_path).unwrap_or_default();
        let partial: Option<RawResults> = std::fs::read(&results_path)
            .ok()
            .and_then(|d| serde_json::from_slice(&d).ok());
        let reason = match out.signal {
            Some(sig) if !out.stalled && !out.timed_out => Stop::Crash(sig),
            _ => Stop::Timeout,
        };
        return Ok(stopped_results()
            .step(step.trim())
            .maybe_partial(partial)
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

/// Runs a Python snippet that prints one JSON document, with a time limit.
/// Used for question authoring (stress-case generators), never for user code.
pub fn python_json(code: &str, limit: Duration) -> Result<Value> {
    let dir = tempfile::tempdir()?;
    let script = dir.path().join("generate.py");
    std::fs::write(&script, code)?;
    let program = interpreter(Language::Python);
    let out = exec()
        .cmd(Command::new(&program).arg(&script))
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
}

fn signal_name(sig: i32) -> &'static str {
    match sig {
        6 => "aborted",
        9 => "killed, often for using too much memory",
        10 | 7 => "bus error",
        11 => "segmentation fault",
        _ => "terminated",
    }
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
        let (r, _) = run_code(
            Language::Python,
            "import ctypes\n\ndef two_sum(nums, target):\n    if len(nums) == 2:\n        ctypes.string_at(0)\n    return []\n",
        );
        // Case 1 (4 numbers) finished; case 2 ([5, 5]) crashed; case 3 not run.
        assert_eq!(statuses(&r), [Status::Fail, Status::Error, Status::NotRun]);
        let err = r.cases[1].error.as_deref().unwrap_or_default();
        assert!(err.contains("crashed"), "{err}");
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
}
