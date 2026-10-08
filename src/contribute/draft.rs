//! A drafted question: the shape the model returns, the prompt that asks for
//! it, and turning it into a validated asset folder. Expected outputs are
//! never taken from the model: they're computed by running the Python
//! reference solution, and every other language's solution must agree.
//! Starter code isn't drafted either: it's generated from the signature
//! (`scaffold::boilerplate`), so it always matches dojo's drivers.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::lang::Language;
use crate::questions::{Bank, Compare, Difficulty, Question};
use crate::runner::{self, Status, Which};
use crate::{scaffold, validate};

/// Folder inside a draft workspace that holds the question asset, laid out
/// like the repo's `questions/` directory so the validator runs unchanged.
pub const QUESTIONS: &str = "questions";

const MAX_GENERATED: usize = 4;
const MAX_TESTS_BYTES: usize = 150_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    /// A JSON object mapping each parameter name to its value.
    pub args_json: String,
    pub hidden: bool,
}

/// What the model returns. Every field is required and every object closed,
/// so one schema works with strict structured output on every provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
    pub companies: Vec<String>,
    pub target_minutes: u32,
    pub function: String,
    pub params: Vec<Param>,
    pub returns: String,
    pub compare: Compare,
    pub statement: String,
    pub hints: Vec<String>,
    pub explanation: String,
    // Reference solutions, one per language (`solution_field`). The
    // defaults let drafts saved before a language was added still load; the
    // missing solution is then reported like any other problem.
    #[serde(default)]
    pub python_solution: String,
    #[serde(default)]
    pub javascript_solution: String,
    #[serde(default)]
    pub typescript_solution: String,
    #[serde(default)]
    pub java_solution: String,
    #[serde(default)]
    pub cpp_solution: String,
    #[serde(default)]
    pub go_solution: String,
    pub tests: Vec<Test>,
    /// Python defining `generate()`, returning a list of argument objects
    /// for large hidden cases. Empty when not needed.
    pub stress_generator_python: String,
}

/// The draft field holding `lang`'s reference solution.
pub fn solution_field(lang: Language) -> String {
    format!("{}_solution", lang.name())
}

fn solution_description(lang: Language) -> &'static str {
    match lang {
        Language::Python => "Python reference solution: a top-level def named exactly `function`",
        Language::JavaScript => {
            "JavaScript reference solution: a top-level camelCase function, no exports or require"
        }
        Language::TypeScript => {
            "TypeScript reference solution: a top-level camelCase function with type annotations, no exports"
        }
        Language::Java => {
            "Java reference solution: `class Solution` with a public camelCase method (LeetCode style)"
        }
        Language::Cpp => {
            "C++ reference solution: `class Solution { public: ... };` with a camelCase method, no #include or main"
        }
        Language::Go => {
            "Go reference solution: starts with `package main`, then a top-level camelCase func, no main"
        }
    }
}

/// The JSON schema the model must follow.
pub fn schema() -> Value {
    let s = |d: &str| json!({ "type": "string", "description": d });
    let mut props = json!({
        "slug": s("kebab-case, unique, e.g. word-ladder"),
        "title": s("short title"),
        "difficulty": { "type": "string", "enum": ["easy", "medium", "hard"] },
        "tags": { "type": "array", "items": s("topic, lowercase kebab-case") },
        "companies": { "type": "array", "items": s("company, lowercase kebab-case") },
        "target_minutes": { "type": "integer", "description": "minutes a prepared candidate needs" },
        "function": s("snake_case function name"),
        "params": {
            "type": "array",
            "items": {
                "type": "object",
                "additionalProperties": false,
                "required": ["name", "type"],
                "properties": { "name": s("snake_case"), "type": s("dojo type") }
            }
        },
        "returns": s("dojo type"),
        "compare": { "type": "string", "enum": ["exact", "unordered", "unordered_deep", "float"] },
        "statement": s("Markdown problem statement"),
        "hints": { "type": "array", "items": s("one progressive hint, Markdown") },
        "explanation": s("Markdown explanation"),
        "tests": {
            "type": "array",
            "items": {
                "type": "object",
                "additionalProperties": false,
                "required": ["args_json", "hidden"],
                "properties": {
                    "args_json": s("JSON object mapping parameter names to values"),
                    "hidden": { "type": "boolean" }
                }
            }
        },
        "stress_generator_python": s("Python defining generate(), or empty")
    });
    let mut required: Vec<String> = [
        "slug",
        "title",
        "difficulty",
        "tags",
        "companies",
        "target_minutes",
        "function",
        "params",
        "returns",
        "compare",
        "statement",
        "hints",
        "explanation",
    ]
    .map(String::from)
    .to_vec();
    for &lang in Language::ALL {
        let field = solution_field(lang);
        props[&field] = s(solution_description(lang));
        required.push(field);
    }
    required.extend(["tests", "stress_generator_python"].map(String::from));
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": required,
        "properties": props,
    })
}

/// Question #1's (Two Sum) reference solutions, for the prompt's worked
/// example in languages the embedded bank doesn't have yet.
fn example_solution(lang: Language) -> &'static str {
    match lang {
        Language::Python | Language::JavaScript => "",
        Language::TypeScript => include_str!("example/typescript.ts"),
        Language::Java => include_str!("example/java.java"),
        Language::Cpp => include_str!("example/cpp.cpp"),
        Language::Go => include_str!("example/go.go"),
    }
}

impl Draft {
    pub fn solution(&self, lang: Language) -> &str {
        match lang {
            Language::Python => &self.python_solution,
            Language::JavaScript => &self.javascript_solution,
            Language::TypeScript => &self.typescript_solution,
            Language::Java => &self.java_solution,
            Language::Cpp => &self.cpp_solution,
            Language::Go => &self.go_solution,
        }
    }

    pub fn solution_mut(&mut self, lang: Language) -> &mut String {
        match lang {
            Language::Python => &mut self.python_solution,
            Language::JavaScript => &mut self.javascript_solution,
            Language::TypeScript => &mut self.typescript_solution,
            Language::Java => &mut self.java_solution,
            Language::Cpp => &mut self.cpp_solution,
            Language::Go => &mut self.go_solution,
        }
    }

    /// Languages whose reference solution is empty.
    pub fn missing(&self) -> Vec<Language> {
        Language::ALL
            .iter()
            .copied()
            .filter(|&l| self.solution(l).trim().is_empty())
            .collect()
    }

    /// An existing question in draft form: the worked example in the prompt.
    pub fn from_question(q: &Question) -> Draft {
        let m = &q.meta;
        let mut d = Draft {
            slug: m.slug.clone(),
            title: m.title.clone(),
            difficulty: m.difficulty,
            tags: m.tags.clone(),
            companies: m.companies.clone(),
            target_minutes: m.target_minutes,
            function: m.signature.function.clone(),
            params: m
                .signature
                .params
                .iter()
                .map(|p| Param {
                    name: p.name.clone(),
                    ty: p.ty.to_string(),
                })
                .collect(),
            returns: m.signature.returns.to_string(),
            compare: m.compare,
            statement: q.statement.clone(),
            hints: q.hints.iter().map(|h| h.body.clone()).collect(),
            explanation: q.explanation.clone(),
            python_solution: String::new(),
            javascript_solution: String::new(),
            typescript_solution: String::new(),
            java_solution: String::new(),
            cpp_solution: String::new(),
            go_solution: String::new(),
            tests: q
                .cases
                .iter()
                .map(|c| Test {
                    args_json: Value::Object(c.input.clone()).to_string(),
                    hidden: c.hidden,
                })
                .collect(),
            stress_generator_python: String::new(),
        };
        for &lang in Language::ALL {
            *d.solution_mut(lang) = q.solutions.get(&lang).cloned().unwrap_or_default();
        }
        d
    }
}

/// The worked example in the prompt: question #1 with a reference solution
/// in every language.
fn example(bank: &Bank) -> Option<Draft> {
    let mut d = Draft::from_question(bank.get(1)?);
    for lang in d.missing() {
        *d.solution_mut(lang) = example_solution(lang).to_string();
    }
    Some(d)
}

/// The system prompt: dojo's format, conventions and a worked example.
pub fn system_prompt(bank: &Bank) -> String {
    let example = example(bank)
        .map(|d| serde_json::to_string_pretty(&d).unwrap_or_default())
        .unwrap_or_default();
    let tags: Vec<String> = bank
        .all()
        .iter()
        .flat_map(|q| q.meta.tags.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let existing: Vec<String> = bank
        .all()
        .iter()
        .map(|q| format!("#{} {} ({})", q.meta.id, q.meta.title, q.meta.slug))
        .collect();
    format!(
        r#"You write coding-interview questions for dojo, an open-source practice tool. A contributor describes a question; you return the complete question as JSON matching the schema. dojo then runs your reference solutions in all six languages (Python, JavaScript, TypeScript, Java, C++, Go) and validates everything, so correctness matters more than prose.

## Format

- Function signature: a snake_case `function`, snake_case `params`, and types from dojo's type system: `int` (32-bit), `long` (64-bit, for values beyond ±2^31, up to ±2^53), `float`, `bool`, `string`, `ListNode`, `TreeNode`, and any of those followed by one or more `[]` (e.g. `int[][]`, `string[]`). No other types (no maps, tuples, or sets): model those as arrays.
- `ListNode` values are JSON arrays of ints; `TreeNode` values are LeetCode-style level-order arrays with nulls. dojo converts them; solutions receive real nodes.
- `compare`: `exact` unless order genuinely doesn't matter (`unordered` for the top-level list, `unordered_deep` at every level) or the answer is a float (`float`, 1e-6 tolerance). Design the problem so the correct answer is unique under the chosen mode.
- `statement`: Markdown in your own words: the problem, then `## Example 1` and `## Example 2` with code blocks showing inputs and output, then `## Constraints` as a bullet list. No title heading.
- `hints`: exactly 3, progressive: a gentle nudge, the key insight, then close to the algorithm.
- `explanation`: Markdown with the approach, a Python code block, `## Complexity`, and `## Pitfalls`.
- `tests`: each `args_json` is a JSON object mapping every parameter name to a value. Do NOT include expected outputs: dojo computes them by running the Python solution. Include the statement's examples and at least 3 visible tests, plus at least 5 hidden tests covering edge cases (smallest inputs, duplicates, negatives, extremes). Keep each literal test small (under ~200 values).
- `stress_generator_python`: when performance matters, Python source defining `generate()` that returns a list of at most {MAX_GENERATED} argument objects for large hidden tests (use `random.Random(<fixed seed>)` so it's deterministic; stay within the stated constraints and keep each case under ~10,000 values). Otherwise an empty string.
- `tags`: reuse existing tags when they fit: {tags}.
- `companies`: companies well known to ask this kind of question, lowercase kebab-case; empty if unsure.
- Don't duplicate an existing question: {existing}.

## Reference solutions (all six languages)

Write one reference solution per language: `python_solution`, `javascript_solution`, `typescript_solution`, `java_solution`, `cpp_solution`, `go_solution`. Each must be optimal, clear and idiomatic, and all six must return identical results for every test (the same algorithm in each is best). Do NOT write starter code: dojo generates it from the signature.

- Python: a plain top-level function named exactly `function` (no class wrapper).
- JavaScript: a plain top-level `function` in camelCase (e.g. `two_sum` becomes `twoSum`), parameters in camelCase, no exports or require.
- TypeScript: like JavaScript, with type annotations on the parameters and the return type.
- Java: LeetCode style, `class Solution {{ public <type> twoSum(...) {{ ... }} }}` (not public, no `main`). `java.util.*`, `java.util.function.*` and `java.util.stream.*` are already imported.
- C++: LeetCode style, `class Solution {{ public: <type> twoSum(...) {{ ... }} }};` with no `#include` and no `main`: every standard header and `using namespace std;` are provided. Take `vector` and `string` parameters by non-const reference (`vector<int>& nums`, `string& s`), scalars and node pointers by value.
- Go: the file starts with `package main`, then a top-level camelCase `func` (no `main`). Import standard packages if needed (`import "sort"`).
- Never define `ListNode` or `TreeNode`: dojo provides them. Fields are `val`/`next` and `val`/`left`/`right` (Go: `Val`/`Next`, `Val`/`Left`/`Right`); null is `None`, `null`, `nullptr` or `nil`.
- Types, per language (Python · JS/TS · Java · C++ · Go):
  - `int`: int · number · int · int · int (32-bit except in Go and Python; keep results in range)
  - `long`: int · number · long · long long · int
  - `float`: float · number · double · double · float64
  - `bool`: bool · boolean · boolean · bool · bool
  - `string`: str · string · String · string · string
  - `T[]`: list[T] · T[] · Java array (`int[]`, `int[][]`, `String[]`) · `vector<T>` · slice `[]T`
  - `ListNode` / `TreeNode`: ListNode · ListNode · ListNode · `ListNode*` · `*ListNode` (same for TreeNode)
- Return arrays exactly as typed: a Java `int[][]` (not a `List`), a C++ `vector<vector<int>>`, a Go `[][]int`. Return an empty array/slice rather than null for empty results.

## Revisions

The contributor may ask for changes, or dojo may report validation problems (naming the file, e.g. `solutions/java.java`). Each time, return the complete question again with the changes applied (never a partial diff), and keep all six reference solutions consistent with each other and with the signature: a change to the problem, signature or behavior must be made in every language. If a language's solution is missing or fails, fix that language without breaking the others.

## Example (question #1, in the exact JSON shape to return)

{example}"#,
        tags = tags.join(", "),
        existing = existing.join("; "),
    )
}

/// How one language fared in the latest build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LangStatus {
    /// The reference solution passes every test; the starter code loads and
    /// doesn't pass.
    Passed,
    /// The draft has no solution in this language.
    Missing,
    /// This many problems name the language's files.
    Failed(usize),
    /// Not run: other problems stopped validation first.
    Unchecked,
}

/// A built draft: the asset folder plus what validation found.
#[derive(Debug, Clone, bon::Builder)]
pub struct Built {
    /// The question folder (`<root>/questions/<NNNN-slug>`).
    pub dir: PathBuf,
    pub problems: Vec<String>,
    pub visible: usize,
    pub hidden: usize,
    pub generated: usize,
    /// Every language, in `Language::ALL` order.
    #[builder(default)]
    pub languages: Vec<(Language, LangStatus)>,
}

/// The language a problem is about, from the file or toolchain it names.
fn about(problem: &str) -> Option<Language> {
    Language::ALL.iter().copied().find(|l| {
        let file = l.asset_file();
        problem.contains(&format!("solutions/{file}"))
            || problem.contains(&format!("boilerplate/{file}"))
            || problem.starts_with(&format!("cannot run {}:", l.label()))
    })
}

fn statuses(draft: &Draft, problems: &[String], validated: bool) -> Vec<(Language, LangStatus)> {
    let missing = draft.missing();
    // The validator only runs code when nothing else is wrong.
    let ran = validated && problems.iter().all(|p| about(p).is_some());
    Language::ALL
        .iter()
        .map(|&l| {
            let failed = problems.iter().filter(|p| about(p) == Some(l)).count();
            let status = if missing.contains(&l) {
                LangStatus::Missing
            } else if failed > 0 {
                LangStatus::Failed(failed)
            } else if ran {
                LangStatus::Passed
            } else {
                LangStatus::Unchecked
            };
            (l, status)
        })
        .collect()
}

/// Writes the asset folder for `draft` under `root`, computes expected
/// outputs, and validates. Problems are returned, not raised, so they can be
/// shown and sent back to the model.
#[bon::builder]
pub fn build(draft: &Draft, root: &Path, id: u32, bank: &Bank) -> Result<Built> {
    let questions = root.join(QUESTIONS);
    let _ = std::fs::remove_dir_all(&questions);
    let dir_name = format!("{id:04}-{}", draft.slug);
    let dir = questions.join(&dir_name);
    for sub in ["boilerplate", "solutions"] {
        std::fs::create_dir_all(dir.join(sub))?;
    }

    let mut problems = duplicates(draft, bank);
    // A missing language is reported, but doesn't stop the others from being
    // checked.
    let missing: Vec<String> = draft
        .missing()
        .into_iter()
        .map(|l| {
            format!(
                "solutions/{} is missing: write the {} reference solution (`{}`)",
                l.asset_file(),
                l.label(),
                solution_field(l)
            )
        })
        .collect();

    // Tests: literal cases, then generated stress cases (hidden).
    let mut cases: Vec<Value> = Vec::new();
    for (i, t) in draft.tests.iter().enumerate() {
        match serde_json::from_str::<Map<String, Value>>(&t.args_json) {
            Ok(input) => cases.push(json!({ "input": input, "output": null, "hidden": t.hidden })),
            Err(e) => problems.push(format!("test {i}: args_json is not a JSON object ({e})")),
        }
    }
    let literal = cases.len();
    if !draft.stress_generator_python.trim().is_empty() {
        let code = format!(
            "{}\n\nimport json as _json\nprint(_json.dumps(generate()))\n",
            draft.stress_generator_python
        );
        match runner::python_json(&code, Duration::from_secs(20)) {
            Ok(Value::Array(list)) => {
                if list.len() > MAX_GENERATED {
                    problems.push(format!(
                        "stress generator returned {} cases; at most {MAX_GENERATED}",
                        list.len()
                    ));
                }
                for (i, input) in list.into_iter().take(MAX_GENERATED).enumerate() {
                    match input {
                        Value::Object(map) => {
                            cases.push(json!({ "input": map, "output": null, "hidden": true }))
                        }
                        _ => problems.push(format!("generated case {i} is not a JSON object")),
                    }
                }
            }
            Ok(_) => problems.push("stress generator must return a list".into()),
            Err(e) => problems.push(format!("stress generator failed: {e:#}")),
        }
    }
    let generated = cases.len() - literal;

    problems.extend(
        write_files()
            .draft(draft)
            .dir(&dir)
            .id(id)
            .cases(&cases)
            .call()?,
    );
    let visible = cases.iter().filter(|c| c["hidden"] == false).count();
    let hidden = cases.len() - visible;
    let built = |mut problems: Vec<String>, validated: bool| {
        // A language the model left out is already explained by `missing`;
        // validation's generic "no `languages.x` entry" would repeat it.
        problems.retain(|p| {
            !Language::ALL.iter().any(|l| {
                *p == format!("meta.json has no `languages.{}` entry", l.name())
                    && missing
                        .iter()
                        .any(|m| m.contains(&format!("solutions/{}", l.asset_file())))
            })
        });
        problems.extend(missing.iter().cloned());
        let languages = statuses(draft, &problems, validated);
        Built::builder()
            .dir(dir.clone())
            .problems(problems)
            .visible(visible)
            .hidden(hidden)
            .generated(generated)
            .languages(languages)
            .build()
    };

    // Expected outputs come from running the Python reference solution.
    if draft.python_solution.trim().is_empty() {
        return Ok(built(problems, false));
    }
    let q = match Bank::load_one(&questions, &dir_name) {
        Ok(q) => q,
        Err(e) => {
            problems.push(format!("{e:#}"));
            return Ok(built(problems, false));
        }
    };
    let python = format!("solutions/{}", Language::Python.asset_file());
    let report = runner::run()
        .question(&q)
        .lang(Language::Python)
        .solution(&dir.join(&python))
        .which(Which::All)
        .call()?;
    if let Some(fatal) = report.fatal {
        problems.push(format!(
            "{python} failed to run (it computes the expected outputs):\n{fatal}"
        ));
        return Ok(built(problems, false));
    }
    for c in &report.cases {
        match (c.status, &c.got) {
            (Status::Error, _) => problems.push(format!(
                "{python} crashed on test {}: {}",
                c.index,
                c.error.as_deref().unwrap_or("error")
            )),
            (Status::Timeout, _) => problems.push(format!(
                "{python} timed out on test {} ({}s limit)",
                c.index,
                runner::CASE_TIMEOUT.as_secs()
            )),
            (_, Some(got)) => cases[c.index]["output"] = got.clone(),
            (_, None) => {}
        }
    }
    let tests = serde_json::to_string_pretty(&json!({
        "$schema": "../../schema/tests.schema.json",
        "cases": cases
    }))? + "\n";
    if tests.len() > MAX_TESTS_BYTES {
        problems.push(format!(
            "tests.json is {} KB; keep it under {} KB (smaller stress cases)",
            tests.len() / 1000,
            MAX_TESTS_BYTES / 1000
        ));
    }
    std::fs::write(dir.join("tests.json"), tests)?;

    // The same checks as `dojo validate` and the per-question tests, for
    // every language the draft has.
    if !problems.is_empty() {
        return Ok(built(problems, false));
    }
    problems.extend(validate::check_question(&questions, &dir_name, true));
    Ok(built(problems, true))
}

fn duplicates(draft: &Draft, bank: &Bank) -> Vec<String> {
    let title = draft.title.to_lowercase();
    bank.all()
        .iter()
        .filter(|q| {
            q.meta.slug == draft.slug
                || q.meta.title.to_lowercase() == title
                || q.meta.signature.function == draft.function
        })
        .map(|q| {
            format!(
                "too close to an existing question: #{} {} ({}); pick a different problem or name",
                q.meta.id, q.meta.title, q.meta.slug
            )
        })
        .collect()
}

/// Writes the question's files: a solution and generated starter code for
/// every language the draft has, all registered in `meta.json`. Returns
/// problems found along the way.
#[bon::builder]
fn write_files(draft: &Draft, dir: &Path, id: u32, cases: &[Value]) -> Result<Vec<String>> {
    let mut problems = Vec::new();
    let signature = json!({
        "function": draft.function,
        "params": draft.params.iter().map(|p| json!({ "name": p.name, "type": p.ty })).collect::<Vec<_>>(),
        "returns": draft.returns,
    });
    let present: Vec<Language> = Language::ALL
        .iter()
        .copied()
        .filter(|&l| !draft.solution(l).trim().is_empty())
        .collect();
    let languages: Map<String, Value> = present
        .iter()
        .map(|l| {
            (
                l.name().to_string(),
                json!({
                    "boilerplate": format!("boilerplate/{}", l.asset_file()),
                    "solution": format!("solutions/{}", l.asset_file()),
                }),
            )
        })
        .collect();
    let meta = json!({
        "$schema": "../../schema/meta.schema.json",
        "id": id,
        "slug": draft.slug,
        "title": draft.title,
        "difficulty": draft.difficulty,
        "tags": draft.tags,
        "companies": draft.companies,
        "target_minutes": draft.target_minutes,
        "signature": signature,
        "compare": draft.compare,
        "statement": "statement.md",
        "hints": "hints.md",
        "explanation": "explanation.md",
        "tests": "tests.json",
        "languages": languages,
    });
    let write = |rel: &str, body: &str| -> Result<()> {
        let body = if body.ends_with('\n') {
            body.to_string()
        } else {
            format!("{body}\n")
        };
        std::fs::write(dir.join(rel), body).with_context(|| format!("writing {rel}"))
    };
    write("meta.json", &serde_json::to_string_pretty(&meta)?)?;
    write("statement.md", draft.statement.trim())?;
    let hints: String = draft
        .hints
        .iter()
        .enumerate()
        .map(|(i, h)| format!("## Hint {}\n{}\n", i + 1, h.trim()))
        .collect::<Vec<_>>()
        .join("\n");
    write("hints.md", &format!("# Hints\n\n{hints}"))?;
    write("explanation.md", draft.explanation.trim())?;
    // Starter code is generated from the signature, never drafted. A
    // signature that doesn't parse is reported when the question loads.
    let scaffold_meta = serde_json::from_value::<scaffold::Meta>(json!({
        "id": id,
        "title": draft.title,
        "difficulty": draft.difficulty,
        "signature": signature,
    }));
    for &lang in &present {
        let file = lang.asset_file();
        if let Ok(m) = &scaffold_meta {
            write(
                &format!("boilerplate/{file}"),
                &scaffold::boilerplate(m, lang),
            )?;
        }
        write(&format!("solutions/{file}"), draft.solution(lang))?;
    }
    if let Err(e) = scaffold_meta {
        problems.push(format!("the signature is invalid: {e}"));
    }
    write(
        "tests.json",
        &serde_json::to_string_pretty(
            &json!({ "$schema": "../../schema/tests.schema.json", "cases": cases }),
        )?,
    )?;
    Ok(problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Climbing Stairs (#13) as a new question, `count_climbs`, with a
    /// reference solution in every language.
    fn count_climbs(bank: &Bank) -> Draft {
        let mut draft = Draft::from_question(bank.get(13).unwrap());
        draft.slug = "count-staircase-climbs".into();
        draft.title = "Count Staircase Climbs".into();
        draft.function = "count_climbs".into();
        draft.python_solution = "def count_climbs(n: int) -> int:\n    prev, curr = 1, 1\n    for _ in range(n - 1):\n        prev, curr = curr, prev + curr\n    return curr\n".into();
        draft.javascript_solution = "function countClimbs(n) {\n  let prev = 1, curr = 1;\n  for (let i = 1; i < n; i++) [prev, curr] = [curr, prev + curr];\n  return curr;\n}\n".into();
        draft.typescript_solution = "function countClimbs(n: number): number {\n  let prev = 1, curr = 1;\n  for (let i = 1; i < n; i++) [prev, curr] = [curr, prev + curr];\n  return curr;\n}\n".into();
        draft.java_solution = "class Solution {\n    public int countClimbs(int n) {\n        int prev = 1, curr = 1;\n        for (int i = 1; i < n; i++) {\n            int next = prev + curr;\n            prev = curr;\n            curr = next;\n        }\n        return curr;\n    }\n}\n".into();
        draft.cpp_solution = "class Solution {\npublic:\n    int countClimbs(int n) {\n        int prev = 1, curr = 1;\n        for (int i = 1; i < n; i++) {\n            int next = prev + curr;\n            prev = curr;\n            curr = next;\n        }\n        return curr;\n    }\n};\n".into();
        draft.go_solution = "package main\n\nfunc countClimbs(n int) int {\n\tprev, curr := 1, 1\n\tfor i := 1; i < n; i++ {\n\t\tprev, curr = curr, prev+curr\n\t}\n\treturn curr\n}\n".into();
        draft
    }

    /// A draft builds into a valid six-language asset, with outputs computed
    /// from its Python solution and starter code generated from the
    /// signature.
    #[test]
    fn builds_a_valid_question_from_a_draft() {
        let bank = Bank::embedded();
        let mut draft = count_climbs(&bank);
        draft.stress_generator_python =
            "def generate():\n    return [{'n': 44}, {'n': 45}]\n".into();

        let root = tempfile::tempdir().unwrap();
        let built = build()
            .draft(&draft)
            .root(root.path())
            .id(18)
            .bank(&bank)
            .call()
            .unwrap();
        assert!(built.problems.is_empty(), "{:#?}", built.problems);
        assert!(built.dir.ends_with("0018-count-staircase-climbs"));
        assert_eq!(built.generated, 2);
        assert_eq!(
            built.languages,
            Language::ALL
                .iter()
                .map(|&l| (l, LangStatus::Passed))
                .collect::<Vec<_>>()
        );
        let tests: Value =
            serde_json::from_str(&std::fs::read_to_string(built.dir.join("tests.json")).unwrap())
                .unwrap();
        let last = tests["cases"].as_array().unwrap().last().unwrap().clone();
        assert_eq!(last["output"], json!(1836311903)); // computed, not given

        let meta: Value =
            serde_json::from_str(&std::fs::read_to_string(built.dir.join("meta.json")).unwrap())
                .unwrap();
        for lang in Language::ALL {
            let file = lang.asset_file();
            assert_eq!(
                meta["languages"][lang.name()]["solution"],
                json!(format!("solutions/{file}"))
            );
            let starter =
                std::fs::read_to_string(built.dir.join(format!("boilerplate/{file}"))).unwrap();
            assert!(starter.contains("18. Count Staircase Climbs"), "{starter}");
        }
    }

    #[test]
    fn reports_wrong_reference_and_duplicates() {
        let bank = Bank::embedded();
        let mut draft = Draft::from_question(bank.get(13).unwrap());
        let root = tempfile::tempdir().unwrap();
        let built = build()
            .draft(&draft)
            .root(root.path())
            .id(18)
            .bank(&bank)
            .call()
            .unwrap();
        let all = built.problems.join("\n");
        assert!(
            all.contains("too close to an existing question: #13"),
            "{all}"
        );

        draft = count_climbs(&bank);
        draft.javascript_solution = "function countClimbs(n) { return n; }".into();
        draft.java_solution = draft.java_solution.replace("return curr;", "return n;");
        let built = build()
            .draft(&draft)
            .root(root.path())
            .id(18)
            .bank(&bank)
            .call()
            .unwrap();
        let all = built.problems.join("\n");
        assert!(all.contains("solutions/javascript.js fails case"), "{all}");
        assert!(all.contains("solutions/java.java fails case"), "{all}");
        let status = |l: Language| built.languages.iter().find(|(x, _)| *x == l).unwrap().1;
        assert!(matches!(status(Language::Java), LangStatus::Failed(_)));
        assert!(matches!(
            status(Language::JavaScript),
            LangStatus::Failed(_)
        ));
        assert_eq!(status(Language::Python), LangStatus::Passed);
        assert_eq!(status(Language::Go), LangStatus::Passed);
    }

    /// A model that leaves a language out (common with small local models)
    /// gets that reported, and the other languages are still checked.
    #[test]
    fn reports_missing_languages_and_checks_the_rest() {
        let bank = Bank::embedded();
        let mut draft = count_climbs(&bank);
        draft.cpp_solution.clear();
        draft.go_solution = "  ".into();
        let root = tempfile::tempdir().unwrap();
        let built = build()
            .draft(&draft)
            .root(root.path())
            .id(18)
            .bank(&bank)
            .call()
            .unwrap();
        assert_eq!(built.problems.len(), 2, "{:#?}", built.problems);
        assert!(built.problems[0].contains("solutions/cpp.cpp is missing"));
        assert!(built.problems[1].contains("`go_solution`"));
        let expected: Vec<(Language, LangStatus)> = Language::ALL
            .iter()
            .map(|&l| match l {
                Language::Cpp | Language::Go => (l, LangStatus::Missing),
                _ => (l, LangStatus::Passed),
            })
            .collect();
        assert_eq!(built.languages, expected);
        assert!(!built.dir.join("solutions/cpp.cpp").exists());
    }

    #[test]
    fn schema_is_closed_and_complete() {
        let s = schema();
        let props = s["properties"].as_object().unwrap();
        assert_eq!(s["required"].as_array().unwrap().len(), props.len());
        assert_eq!(s["additionalProperties"], false);
        for &lang in Language::ALL {
            assert!(props.contains_key(&solution_field(lang)), "{lang}");
        }
        assert!(!props.keys().any(|k| k.contains("boilerplate")));
        // The worked example in the prompt matches the schema's fields and
        // has a solution in every language.
        let example = example(&Bank::embedded()).unwrap();
        assert!(example.missing().is_empty(), "{:?}", example.missing());
        let example = serde_json::to_value(example).unwrap();
        for key in props.keys() {
            assert!(example.get(key).is_some(), "example missing {key}");
        }
    }

    /// The worked example's solutions pass question #1's tests in every
    /// language, so the prompt never teaches a broken convention.
    #[test]
    fn example_solutions_pass() {
        let bank = Bank::embedded();
        let q = bank.get(1).unwrap();
        let example = example(&bank).unwrap();
        let dir = tempfile::tempdir().unwrap();
        for &lang in Language::ALL {
            let path = dir.path().join(lang.solution_file());
            std::fs::write(&path, example.solution(lang)).unwrap();
            let report = runner::run()
                .question(q)
                .lang(lang)
                .solution(&path)
                .which(Which::All)
                .call()
                .unwrap();
            assert!(report.all_passed(), "{lang}: {:?}", report.fatal);
        }
    }
}
