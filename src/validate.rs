//! Question asset validation, shared by `dojo validate` and the test suite
//! (one generated `cargo test` per question folder, see `build.rs`).

use std::collections::{BTreeSet, HashMap};
use std::io::IsTerminal;
use std::path::Path;

use anyhow::{Context, Result};

use crate::lang::Language;
use crate::questions::{Bank, Question};
use crate::runner::{self, Status, Which};

struct Paint(bool);

impl Paint {
    fn green(&self, s: &str) -> String {
        self.wrap("32", s)
    }
    fn red(&self, s: &str) -> String {
        self.wrap("31", s)
    }
    fn dim(&self, s: &str) -> String {
        self.wrap("2", s)
    }
    fn wrap(&self, code: &str, s: &str) -> String {
        if self.0 {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
}

/// Question folder names under `root`, sorted.
pub fn question_dirs(root: &Path) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type()?.is_dir() && !name.starts_with('.') {
            dirs.push(name);
        }
    }
    dirs.sort();
    Ok(dirs)
}

/// Most time a reference solution may take on one case: a third of the
/// per-case limit, so correct user solutions in slower languages (or on
/// slower machines) don't time out.
const SLOW_CASE_MS: f64 = 1000.0;
/// Most time a reference solution may take on all its cases together.
const RUN_BUDGET: std::time::Duration = std::time::Duration::from_secs(20);
const MAX_CASES: usize = 100;
const MAX_TESTS_BYTES: usize = 200_000;
/// Visible cases are printed in the terminal; big inputs belong in hidden ones.
const MAX_VISIBLE_INPUT_CHARS: usize = 1_000;

/// What checking one question found: problems (empty means valid) and a
/// summary of what passed.
pub struct Inspection {
    pub problems: Vec<String>,
    pub passed: Vec<String>,
}

/// Everything wrong with one question folder; empty means valid.
/// `run_code` also runs reference solutions and boilerplates.
pub fn check_question(root: &Path, dir: &str, run_code: bool) -> Vec<String> {
    inspect_question(root, dir, run_code).problems
}

/// Checks one question folder: its files against the JSON Schemas, its
/// organization and content, and (with `run_code`) that it runs safely in
/// every language: reference solutions pass every case, fast and with the
/// same results every time; boilerplate loads and doesn't pass.
pub fn inspect_question(root: &Path, dir: &str, run_code: bool) -> Inspection {
    let folder = root.join(dir);
    let mut passed = Vec::new();
    let mut problems = schema(&folder);
    if problems.is_empty() {
        passed.push("schema".to_string());
    }
    let q = match Bank::load_one(root, dir) {
        Ok(q) => q,
        Err(e) => {
            problems.push(format!("{e:#}"));
            return Inspection { problems, passed };
        }
    };
    let found = problems.len();
    problems.extend(layout(&q));
    problems.extend(content(&q));
    if problems.len() == found {
        passed.push("layout".into());
        passed.push("content".into());
    }
    if run_code && problems.is_empty() {
        let (code_problems, timings) = code(&q, &folder);
        if code_problems.is_empty() {
            passed.push(format!("runs safely ({})", timings.join(", ")));
            passed.push("deterministic".into());
        }
        problems.extend(code_problems);
    }
    Inspection { problems, passed }
}

/// `meta.json` and the tests file against the published JSON Schemas
/// (`schema/*.json`, generated from the same types dojo loads them with).
fn schema(folder: &Path) -> Vec<String> {
    use std::sync::OnceLock;
    static VALIDATORS: OnceLock<Vec<(&'static str, jsonschema::Validator)>> = OnceLock::new();
    let validators = VALIDATORS.get_or_init(|| {
        crate::schema::files()
            .expect("schemas generate")
            .into_iter()
            .map(|(name, body)| {
                let schema: serde_json::Value =
                    serde_json::from_str(&body).expect("schema is JSON");
                (
                    name,
                    jsonschema::validator_for(&schema).expect("schema compiles"),
                )
            })
            .collect()
    });
    let validator = |name: &str| {
        validators
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, v)| v)
            .expect("known schema")
    };

    let mut p = Vec::new();
    let read = |file: &Path| -> Result<serde_json::Value, String> {
        let raw = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
        serde_json::from_str(&raw).map_err(|e| format!("{} is not valid JSON: {e}", file.display()))
    };
    let meta = match read(&folder.join("meta.json")) {
        Ok(v) => v,
        Err(e) => return vec![e],
    };
    for e in validator("meta.schema.json").iter_errors(&meta) {
        p.push(format!(
            "meta.json{}: {e}",
            path_label(&e.instance_path().to_string())
        ));
    }
    if let Some(tests) = meta["tests"].as_str() {
        match read(&folder.join(tests)) {
            Ok(v) => {
                for e in validator("tests.schema.json").iter_errors(&v) {
                    p.push(format!(
                        "{tests}{}: {e}",
                        path_label(&e.instance_path().to_string())
                    ));
                }
            }
            Err(e) => p.push(e),
        }
    }
    p
}

fn path_label(pointer: &str) -> String {
    if pointer.is_empty() {
        String::new()
    } else {
        format!(" at {pointer}")
    }
}

/// Ids and slugs must be unique across the bank.
pub fn check_bank(root: &Path) -> Result<Vec<String>> {
    let (bank, _) = Bank::from_dir(root)?;
    let mut ids: HashMap<u32, Vec<&str>> = HashMap::new();
    let mut slugs: HashMap<&str, Vec<&str>> = HashMap::new();
    for q in bank.all() {
        ids.entry(q.meta.id).or_default().push(&q.dir);
        slugs.entry(&q.meta.slug).or_default().push(&q.dir);
    }
    let mut problems: Vec<String> = ids
        .iter()
        .filter(|(_, dirs)| dirs.len() > 1)
        .map(|(id, dirs)| format!("id {id} is used by {}", dirs.join(", ")))
        .chain(
            slugs
                .iter()
                .filter(|(_, dirs)| dirs.len() > 1)
                .map(|(slug, dirs)| format!("slug `{slug}` is used by {}", dirs.join(", "))),
        )
        .collect();
    problems.sort();
    Ok(problems)
}

/// `dojo validate`. Checks `only` (question folder names) when given, else
/// every question; the bank-wide id/slug check always runs. Returns `true`
/// when everything checked is valid.
pub fn run(root: &Path, only: &[String], run_code: bool) -> Result<bool> {
    let paint = Paint(std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none());
    let all = question_dirs(root)?;
    let dirs: Vec<String> = if only.is_empty() {
        all
    } else {
        for name in only {
            if !all.contains(name) {
                anyhow::bail!("no question folder `{name}` in {}", root.display());
            }
        }
        only.to_vec()
    };
    let mut failed = 0;

    for dir in &dirs {
        let Inspection { problems, passed } = inspect_question(root, dir, run_code);
        if problems.is_empty() {
            let detail = Bank::load_one(root, dir)
                .map(|q| format!("{} cases · {} hints", q.cases.len(), q.hints.len()))
                .unwrap_or_default();
            let detail = format!("{detail} · {}", passed.join(" · "));
            println!("{} {dir}  {}", paint.green("✓"), paint.dim(&detail));
        } else {
            failed += 1;
            println!("{} {dir}", paint.red("✗"));
            print_problems(&problems);
        }
    }

    let bank_problems = check_bank(root)?;
    if !bank_problems.is_empty() {
        failed += 1;
        println!("{} question bank", paint.red("✗"));
        print_problems(&bank_problems);
    }

    println!();
    if failed == 0 {
        println!(
            "{}",
            paint.green(&format!("all {} questions valid", dirs.len()))
        );
    } else {
        println!(
            "{}",
            paint.red(&format!(
                "{failed} problem group(s) in {} questions",
                dirs.len()
            ))
        );
    }
    Ok(failed == 0)
}

fn print_problems(problems: &[String]) {
    for p in problems {
        for (i, line) in p.lines().enumerate() {
            println!("    {}{line}", if i == 0 { "- " } else { "  " });
        }
    }
}

/// Organization: the folder holds exactly what `meta.json` references.
fn layout(q: &Question) -> Vec<String> {
    let m = &q.meta;
    let mut p = Vec::new();

    let expected_dir = format!("{:04}-{}", m.id, m.slug);
    if q.dir != expected_dir {
        p.push(format!("folder should be named `{expected_dir}`"));
    }
    for file in &q.unreferenced {
        p.push(format!("{file} is not referenced by meta.json"));
    }
    for lang in Language::ALL {
        if !m.languages.contains_key(lang) {
            p.push(format!(
                "meta.json has no `languages.{}` entry",
                lang.name()
            ));
        }
    }

    let mut paths = vec![&m.statement, &m.hints, &m.explanation, &m.tests];
    for f in m.languages.values() {
        paths.push(&f.boilerplate);
        paths.push(&f.solution);
    }
    let mut seen = BTreeSet::new();
    for path in paths {
        if !seen.insert(path) {
            p.push(format!("{path} is referenced more than once"));
        }
    }
    for (lang, f) in &m.languages {
        for path in [&f.boilerplate, &f.solution] {
            if !path.ends_with(&format!(".{}", lang.ext())) {
                p.push(format!(
                    "{path} should end in .{} ({})",
                    lang.ext(),
                    lang.label()
                ));
            }
        }
    }
    p
}

/// `/solve` keywords that can't double as tags or companies.
const RESERVED: &[&str] = &["random", "need"];

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c == '_')
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_kebab(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Content: metadata, prose and test cases.
fn content(q: &Question) -> Vec<String> {
    let m = &q.meta;
    let mut p = Vec::new();

    if m.id == 0 {
        p.push("id must be positive".into());
    }
    if !is_kebab(&m.slug) {
        p.push(format!("slug `{}` must be kebab-case", m.slug));
    }
    if m.title.trim().is_empty() {
        p.push("title is empty".into());
    }
    if m.target_minutes == 0 {
        p.push("target_minutes must be positive".into());
    }
    if m.tags.is_empty() {
        p.push("at least one tag is required".into());
    }
    for label in m.tags.iter().chain(&m.companies) {
        if RESERVED.contains(&label.as_str()) {
            p.push(format!("`{label}` is reserved (`/solve {label}`)"));
        }
        if !is_kebab(label) {
            p.push(format!(
                "tag/company `{label}` must be lowercase kebab-case"
            ));
        }
    }

    let sig = &m.signature;
    if !is_ident(&sig.function) {
        p.push(format!("function `{}` must be snake_case", sig.function));
    }
    let mut names = BTreeSet::new();
    for param in &sig.params {
        if !is_ident(&param.name) {
            p.push(format!("param `{}` must be snake_case", param.name));
        }
        if !names.insert(param.name.as_str()) {
            p.push(format!("duplicate param `{}`", param.name));
        }
    }

    if q.statement.trim().is_empty() {
        p.push(format!("{} is empty", m.statement));
    }
    if q.explanation.trim().is_empty() {
        p.push(format!("{} is empty", m.explanation));
    }
    if q.hints.is_empty() {
        p.push(format!("{} needs at least one `## ` hint", m.hints));
    }
    if q.hints.iter().any(|h| h.body.is_empty()) {
        p.push(format!("{} has an empty hint", m.hints));
    }

    let visible = q.cases.iter().filter(|c| !c.hidden).count();
    let hidden = q.cases.len() - visible;
    if visible < 2 {
        p.push(format!("needs at least 2 visible cases (has {visible})"));
    }
    if hidden < 3 {
        p.push(format!("needs at least 3 hidden cases (has {hidden})"));
    }
    for (i, case) in q.cases.iter().enumerate() {
        let keys: BTreeSet<&str> = case.input.keys().map(String::as_str).collect();
        if keys != names {
            p.push(format!(
                "case {i}: input keys {keys:?} do not match params {names:?}"
            ));
            continue;
        }
        for param in &sig.params {
            if let Err(e) = param.ty.check(&case.input[&param.name]) {
                p.push(format!("case {i}: `{}` {e}", param.name));
            }
        }
        if let Err(e) = sig.returns.check(&case.output) {
            p.push(format!("case {i}: output {e}"));
        }
    }

    // Limits that keep the question safe and readable in dojo.
    if q.cases.len() > MAX_CASES {
        p.push(format!("{} test cases; at most {MAX_CASES}", q.cases.len()));
    }
    let size = serde_json::to_string(&q.cases).map_or(0, |t| t.len());
    if size > MAX_TESTS_BYTES {
        p.push(format!(
            "test cases total {} KB; keep them under {} KB (fewer or smaller stress cases)",
            size / 1000,
            MAX_TESTS_BYTES / 1000
        ));
    }
    for (i, case) in q.cases.iter().enumerate().filter(|(_, c)| !c.hidden) {
        let chars = serde_json::Value::Object(case.input.clone())
            .to_string()
            .len();
        if chars > MAX_VISIBLE_INPUT_CHARS {
            p.push(format!(
                "visible case {i} input is {chars} characters; visible cases are printed in the terminal, keep them under {MAX_VISIBLE_INPUT_CHARS} (make big ones hidden)"
            ));
        }
    }
    p
}

/// Code: every reference solution passes every case; every boilerplate
/// loads, defines the function and does not already pass.
fn code(q: &Question, folder: &Path) -> (Vec<String>, Vec<String>) {
    let mut p = Vec::new();
    let mut timings = Vec::new();
    for (lang, files) in &q.meta.languages {
        if let Err(e) = runner::toolchain(*lang) {
            p.push(format!("cannot run {}: {e:#}", lang.label()));
            continue;
        }

        let solution = runner::run()
            .question(q)
            .lang(*lang)
            .solution(&folder.join(&files.solution))
            .which(Which::All)
            .call();
        match solution {
            Err(e) => p.push(format!("{}: {e:#}", files.solution)),
            Ok(report) => {
                if let Some(fatal) = &report.fatal {
                    p.push(format!("{} failed to run:\n{fatal}", files.solution));
                }
                // Fast enough that correct user solutions won't time out.
                for c in report.cases.iter().filter(|c| c.ms > SLOW_CASE_MS) {
                    p.push(format!(
                        "{} took {:.0} ms on case {}; reference solutions must stay under {:.0} ms per case (the limit is {}s) so correct solutions don't time out: make the case smaller",
                        files.solution,
                        c.ms,
                        c.index,
                        SLOW_CASE_MS,
                        runner::CASE_TIMEOUT.as_secs()
                    ));
                }
                if report.elapsed > RUN_BUDGET {
                    p.push(format!(
                        "{} took {:.1}s for all cases; keep it under {}s",
                        files.solution,
                        report.elapsed.as_secs_f64(),
                        RUN_BUDGET.as_secs()
                    ));
                }
                timings.push(format!(
                    "{} {:.2}s",
                    lang.name(),
                    report.elapsed.as_secs_f64()
                ));
                // Same results every time: no randomness or unordered output.
                if report.all_passed() {
                    let again = runner::run()
                        .question(q)
                        .lang(*lang)
                        .solution(&folder.join(&files.solution))
                        .which(Which::All)
                        .call();
                    if let Ok(again) = again {
                        for (a, b) in report.cases.iter().zip(&again.cases) {
                            if a.got != b.got || b.status != Status::Pass {
                                p.push(format!(
                                    "{} gave different results on two runs of case {}: make it deterministic",
                                    files.solution, a.index
                                ));
                            }
                        }
                    }
                }
                for c in report.cases.iter().filter(|c| c.status != Status::Pass) {
                    let expected = &q.cases[c.index].output;
                    let detail = match c.status {
                        Status::Fail => format!(
                            "expected {expected}, got {}",
                            c.got.as_ref().map(|v| v.to_string()).unwrap_or_default()
                        ),
                        Status::Timeout => "timed out".into(),
                        Status::NotRun => "not run (an earlier case timed out or crashed)".into(),
                        _ => c.error.clone().unwrap_or_default(),
                    };
                    p.push(format!(
                        "{} fails case {}: {detail}",
                        files.solution, c.index
                    ));
                }
            }
        }

        let boilerplate = runner::run()
            .question(q)
            .lang(*lang)
            .solution(&folder.join(&files.boilerplate))
            .which(Which::Visible)
            .call();
        match boilerplate {
            Err(e) => p.push(format!("{}: {e:#}", files.boilerplate)),
            Ok(report) => {
                if let Some(fatal) = report.fatal {
                    p.push(format!(
                        "{} must load and define the function:\n{fatal}",
                        files.boilerplate
                    ));
                } else if report.all_passed() {
                    p.push(format!(
                        "{} already passes the visible tests",
                        files.boilerplate
                    ));
                }
            }
        }
    }
    (p, timings)
}

#[cfg(test)]
mod asset_tests {
    use std::path::{Path, PathBuf};

    use rstest::rstest;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("questions")
    }

    /// One test per question folder (each `questions/*/meta.json`), so a
    /// failure names the question. Checks organization, content, and runs
    /// every reference solution and boilerplate. Only with the
    /// `question-tests` feature: question PRs are checked by question.yml.
    #[cfg(feature = "question-tests")]
    #[rstest]
    fn asset(#[files("questions/*/meta.json")] meta: PathBuf) {
        let dir = meta.parent().unwrap();
        let name = dir.file_name().unwrap().to_string_lossy();
        let problems = super::check_question(dir.parent().unwrap(), &name, true);
        assert!(
            problems.is_empty(),
            "{name}:\n  - {}",
            problems.join("\n  - ")
        );
    }

    #[test]
    fn bank_ids_and_slugs_are_unique() {
        let problems = super::check_bank(&root()).unwrap();
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    #[test]
    fn flags_disorganized_assets() {
        let tmp = tempfile::tempdir().unwrap();
        let src = root().join("0001-two-sum");
        let dst = tmp.path().join("0001-two-sum");
        copy_dir(&src, &dst);
        std::fs::write(dst.join("notes.txt"), "stray").unwrap();
        let mut meta: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(dst.join("meta.json")).unwrap()).unwrap();
        meta["languages"]["javascript"]["solution"] = "boilerplate/javascript.js".into();
        std::fs::write(dst.join("meta.json"), meta.to_string()).unwrap();

        let problems = super::check_question(tmp.path(), "0001-two-sum", false);
        let all = problems.join("\n");
        assert!(all.contains("notes.txt is not referenced"), "{all}");
        assert!(
            all.contains("solutions/javascript.js is not referenced"),
            "{all}"
        );
        assert!(
            all.contains("boilerplate/javascript.js is referenced more than once"),
            "{all}"
        );

        meta["languages"]["python"]["boilerplate"] = "../escape.py".into();
        std::fs::write(dst.join("meta.json"), meta.to_string()).unwrap();
        let problems = super::check_question(tmp.path(), "0001-two-sum", false);
        assert!(
            problems[0].contains("inside the question folder"),
            "{problems:?}"
        );
    }

    /// Checks a copy of question 1 after `mutate` breaks it in one way.
    fn broken(mutate: fn(&Path)) -> String {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("0001-two-sum");
        copy_dir(&root().join("0001-two-sum"), &dir);
        mutate(&dir);
        super::check_question(tmp.path(), "0001-two-sum", true).join("\n")
    }

    fn edit_json(file: &Path, change: impl Fn(&mut serde_json::Value)) {
        let mut v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        change(&mut v);
        std::fs::write(file, serde_json::to_string_pretty(&v).unwrap()).unwrap();
    }

    fn bad_slug(dir: &Path) {
        edit_json(&dir.join("meta.json"), |m| m["slug"] = "Two Sum".into());
    }
    fn unknown_case_field(dir: &Path) {
        edit_json(&dir.join("tests.json"), |t| {
            t["cases"][0]["expected"] = 1.into()
        });
    }
    fn zero_target_minutes(dir: &Path) {
        edit_json(&dir.join("meta.json"), |m| m["target_minutes"] = 0.into());
    }
    fn huge_visible_case(dir: &Path) {
        edit_json(&dir.join("tests.json"), |t| {
            let nums: Vec<i64> = (0..600).collect();
            t["cases"][0]["input"]["nums"] = nums.into();
        });
    }
    fn too_many_cases(dir: &Path) {
        edit_json(&dir.join("tests.json"), |t| {
            let case = t["cases"][3].clone();
            let cases = t["cases"].as_array_mut().unwrap();
            cases.extend(std::iter::repeat_n(case, 100));
        });
    }
    fn slow_solution(dir: &Path) {
        std::fs::write(
            dir.join("solutions/python.py"),
            "import time\n\ndef two_sum(nums, target):\n    if len(nums) == 2:\n        time.sleep(1.2)\n    seen = {}\n    for i, x in enumerate(nums):\n        if target - x in seen:\n            return [seen[target - x], i]\n        seen[x] = i\n",
        )
        .unwrap();
    }
    fn nondeterministic_solution(dir: &Path) {
        // Returns the pair in a different order on the second run (passes
        // under `unordered`, but the results differ).
        let marker = dir.join("ran-once");
        std::fs::write(
            dir.join("solutions/python.py"),
            format!(
                "import os\n\ndef two_sum(nums, target):\n    flip = os.path.exists({marker:?})\n    seen = {{}}\n    for i, x in enumerate(nums):\n        if target - x in seen:\n            pair = [seen[target - x], i]\n            open({marker:?}, 'a').close()\n            return pair[::-1] if flip else pair\n        seen[x] = i\n"
            ),
        )
        .unwrap();
    }

    /// Each question-CI check catches what it's for.
    #[rstest]
    #[case::schema_pattern(bad_slug, "meta.json at /slug")]
    #[case::schema_closed_objects(unknown_case_field, "tests.json at /cases/0")]
    #[case::schema_minimum(zero_target_minutes, "meta.json at /target_minutes")]
    #[case::visible_input_size(huge_visible_case, "visible case 0 input")]
    #[case::case_count(too_many_cases, "test cases; at most 100")]
    #[case::slow_reference(slow_solution, "ms on case 1")]
    #[case::nondeterministic(nondeterministic_solution, "different results on two runs")]
    fn question_ci_catches(#[case] mutate: fn(&Path), #[case] expected: &str) {
        let problems = broken(mutate);
        assert!(
            problems.contains(expected),
            "expected `{expected}` in:\n{problems}"
        );
    }

    #[test]
    fn untouched_question_passes_every_check() {
        let i = super::inspect_question(&root(), "0001-two-sum", true);
        assert!(i.problems.is_empty(), "{:?}", i.problems);
        for check in ["schema", "layout", "content", "deterministic"] {
            assert!(
                i.passed.iter().any(|p| p == check),
                "{check} missing from {:?}",
                i.passed
            );
        }
        assert!(
            i.passed
                .iter()
                .any(|p| p.starts_with("runs safely (python"))
        );
    }

    fn copy_dir(src: &Path, dst: &Path) {
        std::fs::create_dir_all(dst).unwrap();
        for entry in std::fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let to = dst.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_dir(&entry.path(), &to);
            } else {
                std::fs::copy(entry.path(), to).unwrap();
            }
        }
    }
}
