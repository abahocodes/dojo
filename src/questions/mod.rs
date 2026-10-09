//! The question bank: schema, loading (embedded or from disk) and lookup.

pub mod search;
pub mod types;

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub use types::Type;

use crate::lang::Language;

/// The bank packed by build.rs: zstd over (path, data) records.
static BANK_ZST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bank.zst"));

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        })
    }
}

/// How often a company asks a question, estimated: 1 rarely, 2
/// occasionally, 3 regularly, 4 frequently, 5 one of their most asked.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct Frequency(#[schemars(range(min = 1, max = 5))] pub u8);

impl Frequency {
    pub fn words(self) -> &'static str {
        match self.0 {
            0 | 1 => "rarely",
            2 => "sometimes",
            3 => "regularly",
            4 => "often",
            _ => "very often",
        }
    }
}

/// Companies that ask a question and how often, by name.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Companies(pub BTreeMap<String, Frequency>);

impl Companies {
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<Frequency> {
        self.0.get(name).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// How often the company that asks it most does.
    pub fn top(&self) -> Option<Frequency> {
        self.0.values().max().copied()
    }

    /// `amazon (very often), google (often)`: most often first.
    pub fn describe(&self) -> String {
        let mut all: Vec<(&String, &Frequency)> = self.0.iter().collect();
        all.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        all.iter()
            .map(|(name, f)| format!("{name} ({})", f.words()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<const N: usize> From<[(&str, u8); N]> for Companies {
    fn from(pairs: [(&str, u8); N]) -> Companies {
        Companies(
            pairs
                .into_iter()
                .map(|(n, f)| (n.to_string(), Frequency(f)))
                .collect(),
        )
    }
}

/// How a returned value is compared against the expected output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Compare {
    /// Values must be identical.
    #[default]
    Exact,
    /// Top-level list order does not matter.
    Unordered,
    /// List order does not matter at any depth.
    UnorderedDeep,
    /// Numbers compared with a 1e-6 tolerance.
    Float,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Param {
    /// snake_case; converted to each language's convention.
    #[schemars(extend("pattern" = "^[a-z_][a-z0-9_]*$"))]
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Signature {
    /// snake_case function name (`two_sum`); JavaScript uses camelCase (`twoSum`).
    #[schemars(extend("pattern" = "^[a-z_][a-z0-9_]*$"))]
    pub function: String,
    pub params: Vec<Param>,
    pub returns: Type,
}

/// `meta.json`: everything about a question except its prose and tests.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// Editor hint pointing at `schema/meta.schema.json`; ignored by dojo.
    #[allow(dead_code)] // editor tooling only
    #[serde(rename = "$schema", default, skip_serializing)]
    pub schema: Option<String>,
    /// Unique, stable question number. The directory is `NNNN-slug`.
    #[schemars(range(min = 1))]
    pub id: u32,
    /// kebab-case, unique.
    #[schemars(extend("pattern" = "^[a-z0-9]+(-[a-z0-9]+)*$"))]
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    /// Topics (`arrays`, `dfs`, ...), lowercase kebab-case.
    #[schemars(length(min = 1))]
    pub tags: Vec<String>,
    /// Companies known to ask this question (lowercase kebab-case), each
    /// with an estimate of how often: 1 rarely, 5 one of their most asked.
    #[serde(default)]
    pub companies: Companies,
    /// Time a well-prepared candidate should need.
    #[schemars(range(min = 1))]
    pub target_minutes: u32,
    pub signature: Signature,
    #[serde(default)]
    pub compare: Compare,
    /// Problem statement (Markdown), relative to this folder.
    pub statement: String,
    /// Hints (Markdown, one `## ` section per hint), relative to this folder.
    pub hints: String,
    /// Explained solution (Markdown), relative to this folder.
    pub explanation: String,
    /// Test cases (`tests.schema.json`), relative to this folder.
    pub tests: String,
    /// Per-language files. A question supports exactly the languages listed.
    #[schemars(schema_with = "languages_schema")]
    pub languages: BTreeMap<Language, LanguageFiles>,
}

/// One language's files, relative to the question folder.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LanguageFiles {
    /// What the user starts from; copied as-is into their working file.
    pub boilerplate: String,
    /// Reference solution; must pass every test.
    pub solution: String,
}

fn languages_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let files = generator.subschema_for::<LanguageFiles>();
    let properties: serde_json::Map<String, Value> = Language::ALL
        .iter()
        .map(|l| (l.name().to_string(), files.clone().to_value()))
        .collect();
    schemars::json_schema!({
        "type": "object",
        "properties": properties,
        "additionalProperties": false,
        "minProperties": 1
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Case {
    /// Argument values keyed by param name, encoded as JSON (`ListNode` as an
    /// array, `TreeNode` as a level-order array with nulls).
    pub input: Map<String, Value>,
    /// Expected return value, encoded the same way.
    pub output: Value,
    /// Hidden cases only run on `/submit`.
    #[serde(default)]
    pub hidden: bool,
}

/// `tests.json`: the question's test cases.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TestsFile {
    #[allow(dead_code)] // editor tooling only
    #[serde(rename = "$schema", default, skip_serializing)]
    pub schema: Option<String>,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone)]
pub struct Hint {
    #[allow(dead_code)] // shown by `/hint` once hints get titles beyond "Hint N"
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct Question {
    /// Directory name inside `questions/`, e.g. `0001-two-sum`.
    pub dir: String,
    pub meta: Meta,
    pub statement: String,
    pub hints: Vec<Hint>,
    pub explanation: String,
    pub cases: Vec<Case>,
    /// Starter code users begin from, per language.
    pub boilerplate: BTreeMap<Language, String>,
    /// Reference solutions, per language.
    pub solutions: BTreeMap<Language, String>,
    /// Files in the folder that `meta.json` doesn't reference.
    pub unreferenced: Vec<String>,
}

impl Question {
    pub fn id(&self) -> u32 {
        self.meta.id
    }

    pub fn supports(&self, lang: Language) -> bool {
        self.meta.languages.contains_key(&lang)
    }
}

#[derive(Debug)]
pub struct LoadError {
    pub dir: String,
    pub message: String,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.dir, self.message)
    }
}

#[derive(Debug, Default)]
pub struct Bank {
    questions: Vec<Question>,
}

impl Bank {
    /// The question bank compiled into the binary.
    pub fn embedded() -> Bank {
        let files = unpack(BANK_ZST).unwrap_or_default();
        // Embedded questions are validated in CI; a bad one is skipped rather
        // than taking the whole app down.
        Bank::from_files(files).0
    }

    /// Loads a bank from a `questions/` directory on disk (for `dojo validate`
    /// and question authors).
    pub fn from_dir(root: &Path) -> Result<(Bank, Vec<LoadError>)> {
        let mut files = BTreeMap::new();
        collect_files(root, root, &mut files)
            .with_context(|| format!("reading {}", root.display()))?;
        Ok(Bank::from_files(files))
    }

    /// Loads a single question folder from disk.
    pub fn load_one(root: &Path, dir: &str) -> Result<Question> {
        let mut files = BTreeMap::new();
        collect_files(&root.join(dir), &root.join(dir), &mut files)
            .with_context(|| format!("reading {dir}"))?;
        load_question(dir, &files)
    }

    fn from_files(files: BTreeMap<String, Vec<u8>>) -> (Bank, Vec<LoadError>) {
        let mut by_dir: BTreeMap<String, BTreeMap<String, Vec<u8>>> = BTreeMap::new();
        for (path, data) in files {
            let Some((dir, rest)) = path.split_once('/') else {
                continue; // top-level files such as README.md
            };
            by_dir
                .entry(dir.to_string())
                .or_default()
                .insert(rest.to_string(), data);
        }

        let mut questions = Vec::new();
        let mut errors = Vec::new();
        for (dir, files) in by_dir {
            match load_question(&dir, &files) {
                Ok(q) => questions.push(q),
                Err(e) => errors.push(LoadError {
                    dir,
                    message: format!("{e:#}"),
                }),
            }
        }
        questions.sort_by_key(|q| q.meta.id);
        (Bank { questions }, errors)
    }

    pub fn all(&self) -> &[Question] {
        &self.questions
    }

    pub fn get(&self, id: u32) -> Option<&Question> {
        self.questions
            .binary_search_by_key(&id, |q| q.meta.id)
            .ok()
            .map(|i| &self.questions[i])
    }

    pub fn by_slug(&self, slug: &str) -> Option<&Question> {
        self.questions.iter().find(|q| q.meta.slug == slug)
    }

    /// Every distinct tag and company, sorted.
    pub fn labels(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .questions
            .iter()
            .flat_map(|q| q.meta.tags.iter().chain(q.meta.companies.names()).cloned())
            .collect();
        out.sort();
        out.dedup();
        out
    }
}

fn collect_files(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        if entry.file_type()?.is_dir() {
            collect_files(root, &path, out)?;
        } else {
            let rel = path
                .strip_prefix(root)?
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            out.insert(rel, std::fs::read(&path)?);
        }
    }
    Ok(())
}

fn text(files: &BTreeMap<String, Vec<u8>>, path: &str) -> Result<String> {
    let data = files.get(path).with_context(|| format!("missing {path}"))?;
    String::from_utf8(data.clone()).with_context(|| format!("{path} is not UTF-8"))
}

/// Checks a path from `meta.json` stays inside the question folder.
fn check_path(path: &str) -> Result<()> {
    let bad = path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..");
    if bad {
        anyhow::bail!("`{path}` must be a relative path inside the question folder");
    }
    Ok(())
}

/// Loads one question folder. `meta.json` is the manifest: it names every
/// other file, so file names carry no meaning.
fn load_question(dir: &str, files: &BTreeMap<String, Vec<u8>>) -> Result<Question> {
    let meta: Meta =
        serde_json::from_str(&text(files, "meta.json")?).context("invalid meta.json")?;

    let mut referenced = vec!["meta.json".to_string()];
    let mut read = |path: &str| -> Result<String> {
        check_path(path)?;
        referenced.push(path.to_string());
        text(files, path)
    };

    let statement = read(&meta.statement)?;
    let hints = parse_hints(&read(&meta.hints)?);
    let explanation = read(&meta.explanation)?;
    let tests: TestsFile = serde_json::from_str(&read(&meta.tests)?)
        .with_context(|| format!("invalid {}", meta.tests))?;

    let mut boilerplate = BTreeMap::new();
    let mut solutions = BTreeMap::new();
    for (lang, f) in &meta.languages {
        boilerplate.insert(*lang, read(&f.boilerplate)?);
        solutions.insert(*lang, read(&f.solution)?);
    }

    let unreferenced = files
        .keys()
        .filter(|path| !referenced.contains(path))
        .cloned()
        .collect();

    Ok(Question::builder()
        .dir(dir.to_string())
        .meta(meta)
        .statement(statement)
        .hints(hints)
        .explanation(explanation)
        .cases(tests.cases)
        .boilerplate(boilerplate)
        .solutions(solutions)
        .unreferenced(unreferenced)
        .build())
}

/// Splits `hints.md` on `## ` headings. Each heading starts a new hint.
fn parse_hints(md: &str) -> Vec<Hint> {
    let mut hints: Vec<Hint> = Vec::new();
    for line in md.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            hints.push(Hint {
                title: title.trim().to_string(),
                body: String::new(),
            });
        } else if let Some(h) = hints.last_mut() {
            h.body.push_str(line);
            h.body.push('\n');
        }
    }
    for h in &mut hints {
        h.body = h.body.trim().to_string();
    }
    hints
}

/// Reads build.rs's archive back into (path, data) pairs.
fn unpack(packed: &[u8]) -> Option<BTreeMap<String, Vec<u8>>> {
    let raw = zstd::decode_all(packed).ok()?;
    let mut files = BTreeMap::new();
    let mut rest = raw.as_slice();
    fn take(n: usize, rest: &mut &[u8]) -> Option<Vec<u8>> {
        let (head, tail) = rest.split_at_checked(n)?;
        *rest = tail;
        Some(head.to_vec())
    }
    while !rest.is_empty() {
        let len = u32::from_le_bytes(take(4, &mut rest)?.try_into().ok()?) as usize;
        let path = String::from_utf8(take(len, &mut rest)?).ok()?;
        let len = u32::from_le_bytes(take(4, &mut rest)?.try_into().ok()?) as usize;
        files.insert(path, take(len, &mut rest)?);
    }
    Some(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_hints() {
        let hints = parse_hints("# Hints\n\n## Hint 1\nfirst\n\n## Hint 2\nsecond\nmore\n");
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].title, "Hint 1");
        assert_eq!(hints[0].body, "first");
        assert_eq!(hints[1].body, "second\nmore");
    }

    #[test]
    fn embedded_bank_loads() {
        let files = unpack(BANK_ZST).expect("bank unpacks");
        let (bank, errors) = Bank::from_files(files);
        assert!(errors.is_empty(), "{errors:?}");
        assert!(!bank.all().is_empty());
    }

    #[test]
    fn embedded_bank_matches_the_questions_folder() {
        let embedded = unpack(BANK_ZST).expect("bank unpacks");
        let mut on_disk = BTreeMap::new();
        collect_files(Path::new("questions"), Path::new("questions"), &mut on_disk).unwrap();
        // Top-level files (README.md) aren't part of any question.
        on_disk.retain(|path, _| path.contains('/'));
        assert_eq!(
            embedded.keys().collect::<Vec<_>>(),
            on_disk.keys().collect::<Vec<_>>()
        );
        assert!(embedded.iter().all(|(path, data)| on_disk[path] == *data));
    }

    #[test]
    fn unpack_rejects_truncated_archives() {
        let mut archive = Vec::new();
        archive.extend_from_slice(&5u32.to_le_bytes());
        archive.extend_from_slice(b"a/b");
        let packed = zstd::encode_all(archive.as_slice(), 1).unwrap();
        assert!(unpack(&packed).is_none());
        assert!(unpack(b"not zstd").is_none());
    }
}
