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
}

#[derive(Debug, Clone)]
pub struct CaseResult {
    pub index: usize,
    pub hidden: bool,
    pub status: Status,
    pub got: Option<Value>,
    pub error: Option<String>,
    pub stdout: Option<String>,
    pub ms: f64,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Deserialize)]
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
    ms: f64,
}

#[derive(Debug, Deserialize)]
struct RawResults {
    #[serde(default)]
    fatal: Option<String>,
    #[serde(default)]
    load_stdout: Option<String>,
    #[serde(default)]
    results: Vec<RawResult>,
}

pub fn run(q: &Question, lang: Language, solution: &Path, which: Which) -> Result<RunReport> {
    run_cancellable(q, lang, solution, which, &AtomicBool::new(false))
}

/// Like `run`, but stops the harness as soon as `cancel` is set.
pub fn run_cancellable(
    q: &Question,
    lang: Language,
    solution: &Path,
    which: Which,
    cancel: &AtomicBool,
) -> Result<RunReport> {
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
    let raw = run_harness(q, lang, solution, &indices, cancel)?;
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
                _ => Status::Error,
            };
            CaseResult {
                index: r.index,
                hidden: case.hidden,
                status,
                got: r.got,
                error: r.error,
                stdout: r.stdout,
                ms: r.ms,
            }
        })
        .collect();

    Ok(RunReport {
        which,
        cases,
        fatal: raw.fatal,
        load_stdout: raw.load_stdout.filter(|s| !s.trim().is_empty()),
        elapsed,
    })
}

fn run_harness(
    q: &Question,
    lang: Language,
    solution: &Path,
    indices: &[usize],
    cancel: &AtomicBool,
) -> Result<RawResults> {
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
            cmd.env("NODE_OPTIONS", "");
        }
    }
    cmd.arg(&harness)
        .arg(&solution)
        .arg(&spec_path)
        .arg(&results_path);

    let limit = CASE_TIMEOUT * indices.len() as u32 + Duration::from_secs(5);
    let out = exec(&mut cmd, limit, cancel)
        .map_err(|e| anyhow!("could not start {program} ({e}); {}", install_hint(lang)))?;

    if out.timed_out {
        return Ok(RawResults {
            fatal: Some(format!("run exceeded {}s and was stopped", limit.as_secs())),
            load_stdout: None,
            results: vec![],
        });
    }
    match std::fs::read(&results_path) {
        Ok(data) => Ok(serde_json::from_slice(&data).context("harness wrote invalid results")?),
        Err(_) if !out.success => Ok(RawResults {
            fatal: Some(format!("{}{}", out.stdout, out.stderr).trim().to_string()),
            load_stdout: None,
            results: vec![],
        }),
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
    let out = exec(
        Command::new(&program).arg(&script),
        limit,
        &AtomicBool::new(false),
    )
    .map_err(|e| anyhow!("could not start {program} ({e})"))?;
    if out.timed_out {
        bail!("took longer than {}s", limit.as_secs());
    }
    if !out.success {
        bail!("{}", out.stderr.trim());
    }
    serde_json::from_str(out.stdout.trim()).context("did not print valid JSON")
}

/// Output of a finished child process.
struct Exec {
    success: bool,
    stdout: String,
    stderr: String,
    timed_out: bool,
}

/// Runs a command with a wall-clock limit, capturing output without risking a
/// full-pipe deadlock.
fn exec(cmd: &mut Command, limit: Duration, cancel: &AtomicBool) -> Result<Exec> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut out = child.stdout.take().context("no stdout")?;
    let mut err = child.stderr.take().context("no stderr")?;
    let out_t = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = out.read_to_string(&mut s);
        s
    });
    let err_t = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = err.read_to_string(&mut s);
        s
    });

    let deadline = Instant::now() + limit;
    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (Some(status), false);
        }
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("cancelled");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break (None, true);
        }
        std::thread::sleep(Duration::from_millis(10));
    };

    Ok(Exec {
        success: status.is_some_and(|s| s.success()),
        stdout: out_t.join().unwrap_or_default(),
        stderr: err_t.join().unwrap_or_default(),
        timed_out,
    })
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
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn compare_modes() {
        assert!(matches(Compare::Exact, &json!([1, 2]), &json!([1, 2])));
        assert!(!matches(Compare::Exact, &json!([1, 2]), &json!([2, 1])));
        assert!(matches(Compare::Unordered, &json!([1, 2]), &json!([2, 1])));
        assert!(!matches(
            Compare::Unordered,
            &json!([[1, 2]]),
            &json!([[2, 1]])
        ));
        assert!(matches(
            Compare::UnorderedDeep,
            &json!([[1, 2], [3]]),
            &json!([[3], [2, 1]])
        ));
        assert!(matches(Compare::Float, &json!(0.1), &json!(0.1000000001)));
        assert!(!matches(Compare::Float, &json!(0.1), &json!(0.2)));
    }
}
