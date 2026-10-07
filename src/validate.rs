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

/// Everything wrong with one question folder; empty means valid.
/// `run_code` also runs reference solutions and boilerplates.
pub fn check_question(root: &Path, dir: &str, run_code: bool) -> Vec<String> {
    let q = match Bank::load_one(root, dir) {
        Ok(q) => q,
        Err(e) => return vec![format!("{e:#}")],
    };
    let mut problems = layout(&q);
    problems.extend(content(&q));
    if run_code && problems.is_empty() {
        problems.extend(code(&q, &root.join(dir)));
    }
    problems
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

/// `dojo validate`. Returns `true` when every question is valid.
pub fn run(root: &Path, run_code: bool) -> Result<bool> {
    let paint = Paint(std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none());
    let dirs = question_dirs(root)?;
    let mut failed = 0;

    for dir in &dirs {
        let problems = check_question(root, dir, run_code);
        if problems.is_empty() {
            let detail = Bank::load_one(root, dir)
                .map(|q| {
                    format!(
                        "{} cases · {} hints · {}",
                        q.cases.len(),
                        q.hints.len(),
                        q.languages()
                            .iter()
                            .map(|l| l.name())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .unwrap_or_default();
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
    p
}

/// Code: every reference solution passes every case; every boilerplate
/// loads, defines the function and does not already pass.
fn code(q: &Question, folder: &Path) -> Vec<String> {
    let mut p = Vec::new();
    for (lang, files) in &q.meta.languages {
        if let Err(e) = runner::toolchain(*lang) {
            p.push(format!("cannot run {}: {e:#}", lang.label()));
            continue;
        }

        match runner::run(q, *lang, &folder.join(&files.solution), Which::All) {
            Err(e) => p.push(format!("{}: {e:#}", files.solution)),
            Ok(report) => {
                if let Some(fatal) = report.fatal {
                    p.push(format!("{} failed to run:\n{fatal}", files.solution));
                }
                for c in report.cases.iter().filter(|c| c.status != Status::Pass) {
                    let expected = &q.cases[c.index].output;
                    let detail = match c.status {
                        Status::Fail => format!(
                            "expected {expected}, got {}",
                            c.got.as_ref().map(|v| v.to_string()).unwrap_or_default()
                        ),
                        Status::Timeout => "timed out".into(),
                        _ => c.error.clone().unwrap_or_default(),
                    };
                    p.push(format!(
                        "{} fails case {}: {detail}",
                        files.solution, c.index
                    ));
                }
            }
        }

        match runner::run(q, *lang, &folder.join(&files.boilerplate), Which::Visible) {
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
    p
}

#[cfg(test)]
mod asset_tests {
    use std::path::{Path, PathBuf};

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("questions")
    }

    /// Asserts one question folder is fully valid, including running code.
    fn check_asset(dir: &str) {
        let problems = super::check_question(&root(), dir, true);
        assert!(
            problems.is_empty(),
            "{dir}:\n  - {}",
            problems.join("\n  - ")
        );
    }

    // One `#[test] fn asset_<dir>()` per question folder, generated by build.rs.
    include!(concat!(env!("OUT_DIR"), "/asset_tests.rs"));

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
