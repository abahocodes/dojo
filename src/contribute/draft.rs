//! A drafted question: the shape the model returns, the prompt that asks for
//! it, and turning it into a validated asset folder. Expected outputs are
//! never taken from the model: they're computed by running the Python
//! reference solution, and the JavaScript one must agree.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::lang::Language;
use crate::questions::{Bank, Compare, Difficulty, Question};
use crate::runner::{self, Status, Which};
use crate::validate;

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
/// so one schema works with strict structured output on both providers.
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
    pub python_boilerplate: String,
    pub python_solution: String,
    pub javascript_boilerplate: String,
    pub javascript_solution: String,
    pub tests: Vec<Test>,
    /// Python defining `generate()`, returning a list of argument objects
    /// for large hidden cases. Empty when not needed.
    pub stress_generator_python: String,
}

/// The JSON schema the model must follow.
pub fn schema() -> Value {
    let s = |d: &str| json!({ "type": "string", "description": d });
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": [
            "slug", "title", "difficulty", "tags", "companies", "target_minutes",
            "function", "params", "returns", "compare", "statement", "hints",
            "explanation", "python_boilerplate", "python_solution",
            "javascript_boilerplate", "javascript_solution", "tests",
            "stress_generator_python"
        ],
        "properties": {
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
            "python_boilerplate": s("Python starter code"),
            "python_solution": s("Python reference solution"),
            "javascript_boilerplate": s("JavaScript starter code"),
            "javascript_solution": s("JavaScript reference solution"),
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
        }
    })
}

impl Draft {
    /// An existing question in draft form: the worked example in the prompt.
    pub fn from_question(q: &Question) -> Draft {
        let m = &q.meta;
        let code = |map: &std::collections::BTreeMap<Language, String>, l| {
            map.get(&l).cloned().unwrap_or_default()
        };
        Draft {
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
            python_boilerplate: code(&q.boilerplate, Language::Python),
            python_solution: code(&q.solutions, Language::Python),
            javascript_boilerplate: code(&q.boilerplate, Language::JavaScript),
            javascript_solution: code(&q.solutions, Language::JavaScript),
            tests: q
                .cases
                .iter()
                .map(|c| Test {
                    args_json: Value::Object(c.input.clone()).to_string(),
                    hidden: c.hidden,
                })
                .collect(),
            stress_generator_python: String::new(),
        }
    }
}

/// The system prompt: dojo's format, conventions and a worked example.
pub fn system_prompt(bank: &Bank) -> String {
    // The example's boilerplate headers use id 0, like new drafts must.
    let example = bank
        .get(1)
        .map(|q| {
            let mut d = Draft::from_question(q);
            d.python_boilerplate = d.python_boilerplate.replacen("1. ", "0. ", 1);
            d.javascript_boilerplate = d.javascript_boilerplate.replacen("1. ", "0. ", 1);
            serde_json::to_string_pretty(&d).unwrap_or_default()
        })
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
        r#"You write coding-interview questions for dojo, an open-source practice tool. A contributor describes a question; you return the complete question as JSON matching the schema. dojo then runs your reference solutions and validates everything, so correctness matters more than prose.

## Format

- Function signature: a snake_case `function`, snake_case `params`, and types from dojo's type system: `int`, `float`, `bool`, `string`, `ListNode`, `TreeNode`, and any of those followed by one or more `[]` (e.g. `int[][]`, `string[]`). No other types (no maps, tuples, or sets): model those as arrays.
- `ListNode` values are JSON arrays of ints; `TreeNode` values are LeetCode-style level-order arrays with nulls. dojo converts them; solutions receive real nodes with `.val/.next` and `.val/.left/.right`.
- `compare`: `exact` unless order genuinely doesn't matter (`unordered` for the top-level list, `unordered_deep` at every level) or the answer is a float (`float`, 1e-6 tolerance). Design the problem so the correct answer is unique under the chosen mode.
- `statement`: Markdown in your own words: the problem, then `## Example 1` and `## Example 2` with code blocks showing inputs and output, then `## Constraints` as a bullet list. No title heading.
- `hints`: exactly 3, progressive: a gentle nudge, the key insight, then close to the algorithm.
- `explanation`: Markdown with the approach, a Python code block, `## Complexity`, and `## Pitfalls`.
- Python: a plain top-level function named exactly `function` (no class wrapper). JavaScript: a plain top-level `function` in camelCase (e.g. `two_sum` becomes `twoSum`), parameters in camelCase, no exports or require. Never define ListNode/TreeNode in solutions; the boilerplate may define them in Python and must only describe them in a comment in JavaScript.
- Boilerplate: start with a two-line comment header like the example (`<id>. <title> (<difficulty>)` with id `0`, then the hint line), then the empty function (`pass` in Python, an empty body in JavaScript). Boilerplate must load without errors and must not solve the problem.
- Reference solutions: optimal, clear, idiomatic. Python and JavaScript must return identical results for every test.
- `tests`: each `args_json` is a JSON object mapping every parameter name to a value. Do NOT include expected outputs: dojo computes them by running the Python solution. Include the statement's examples and at least 3 visible tests, plus at least 5 hidden tests covering edge cases (smallest inputs, duplicates, negatives, extremes). Keep each literal test small (under ~200 values).
- `stress_generator_python`: when performance matters, Python source defining `generate()` that returns a list of at most {MAX_GENERATED} argument objects for large hidden tests (use `random.Random(<fixed seed>)` so it's deterministic; stay within the stated constraints and keep each case under ~10,000 values). Otherwise an empty string.
- `tags`: reuse existing tags when they fit: {tags}.
- `companies`: companies well known to ask this kind of question, lowercase kebab-case; empty if unsure.
- Don't duplicate an existing question: {existing}.

## Revisions

The contributor may ask for changes, or dojo may report validation problems. Each time, return the complete question again with the changes applied (never a partial diff).

## Example (question #1, in the exact JSON shape to return)

{example}"#,
        tags = tags.join(", "),
        existing = existing.join("; "),
    )
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

    write_files()
        .draft(draft)
        .dir(&dir)
        .id(id)
        .cases(&cases)
        .call()?;
    let visible = cases.iter().filter(|c| c["hidden"] == false).count();
    let hidden = cases.len() - visible;
    let built = |problems| {
        Built::builder()
            .dir(dir.clone())
            .problems(problems)
            .visible(visible)
            .hidden(hidden)
            .generated(generated)
            .build()
    };

    // Expected outputs come from running the Python reference solution.
    let q = match Bank::load_one(&questions, &dir_name) {
        Ok(q) => q,
        Err(e) => {
            problems.push(format!("{e:#}"));
            return Ok(built(problems));
        }
    };
    let report = runner::run()
        .question(&q)
        .lang(Language::Python)
        .solution(&dir.join("solutions/python.py"))
        .which(Which::All)
        .call()?;
    if let Some(fatal) = report.fatal {
        problems.push(format!("the Python solution failed to run:\n{fatal}"));
        return Ok(built(problems));
    }
    for c in &report.cases {
        match (c.status, &c.got) {
            (Status::Error, _) => problems.push(format!(
                "the Python solution crashed on test {}: {}",
                c.index,
                c.error.as_deref().unwrap_or("error")
            )),
            (Status::Timeout, _) => problems.push(format!(
                "the Python solution timed out on test {} ({}s limit)",
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

    // The same checks as `dojo validate` and the per-question tests.
    if problems.is_empty() {
        problems.extend(validate::check_question(&questions, &dir_name, true));
    }
    Ok(built(problems))
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

#[bon::builder]
fn write_files(draft: &Draft, dir: &Path, id: u32, cases: &[Value]) -> Result<()> {
    let meta = json!({
        "$schema": "../../schema/meta.schema.json",
        "id": id,
        "slug": draft.slug,
        "title": draft.title,
        "difficulty": draft.difficulty,
        "tags": draft.tags,
        "companies": draft.companies,
        "target_minutes": draft.target_minutes,
        "signature": {
            "function": draft.function,
            "params": draft.params.iter().map(|p| json!({ "name": p.name, "type": p.ty })).collect::<Vec<_>>(),
            "returns": draft.returns,
        },
        "compare": draft.compare,
        "statement": "statement.md",
        "hints": "hints.md",
        "explanation": "explanation.md",
        "tests": "tests.json",
        "languages": {
            "python": { "boilerplate": "boilerplate/python.py", "solution": "solutions/python.py" },
            "javascript": { "boilerplate": "boilerplate/javascript.js", "solution": "solutions/javascript.js" }
        }
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
    // Boilerplate headers are drafted with id 0 ("# 0. Title"); stamp the real id.
    let header = |code: &str| {
        let mut out = String::new();
        for (i, line) in code.lines().enumerate() {
            let stamped = match (i, line.trim_start().strip_prefix(['#', '/'])) {
                (0, Some(_)) => line.replacen(" 0. ", &format!(" {id}. "), 1),
                _ => line.to_string(),
            };
            out.push_str(&stamped);
            out.push('\n');
        }
        out
    };
    write("boilerplate/python.py", &header(&draft.python_boilerplate))?;
    write(
        "boilerplate/javascript.js",
        &header(&draft.javascript_boilerplate),
    )?;
    write("solutions/python.py", &draft.python_solution)?;
    write("solutions/javascript.js", &draft.javascript_solution)?;
    write(
        "tests.json",
        &serde_json::to_string_pretty(
            &json!({ "$schema": "../../schema/tests.schema.json", "cases": cases }),
        )?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An existing question round-trips through the draft form and builds
    /// into a valid asset with outputs computed from its Python solution.
    #[test]
    fn builds_a_valid_question_from_a_draft() {
        let bank = Bank::embedded();
        let mut draft = Draft::from_question(bank.get(13).unwrap());
        // Pretend it's new so the duplicate check passes.
        draft.slug = "count-staircase-climbs".into();
        draft.title = "Count Staircase Climbs".into();
        draft.function = "count_climbs".into();
        draft.python_solution = draft
            .python_solution
            .replace("climb_stairs", "count_climbs");
        draft.python_boilerplate = draft
            .python_boilerplate
            .replace("climb_stairs", "count_climbs");
        draft.javascript_solution = draft
            .javascript_solution
            .replace("climbStairs", "countClimbs");
        draft.javascript_boilerplate = draft
            .javascript_boilerplate
            .replace("climbStairs", "countClimbs");
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
        let tests: Value =
            serde_json::from_str(&std::fs::read_to_string(built.dir.join("tests.json")).unwrap())
                .unwrap();
        let last = tests["cases"].as_array().unwrap().last().unwrap().clone();
        assert_eq!(last["output"], json!(1836311903)); // computed, not given
    }

    #[test]
    fn reports_wrong_reference_and_duplicates() {
        let bank = Bank::embedded();
        let mut draft = Draft::from_question(bank.get(13).unwrap());
        draft.javascript_solution = "function climbStairs(n) { return n; }".into();
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

        draft.slug = "x-stairs".into();
        draft.title = "X".into();
        draft.function = "x_stairs".into();
        draft.python_solution = draft.python_solution.replace("climb_stairs", "x_stairs");
        draft.python_boilerplate = draft.python_boilerplate.replace("climb_stairs", "x_stairs");
        draft.javascript_solution = "function xStairs(n) { return n; }".into();
        draft.javascript_boilerplate = draft
            .javascript_boilerplate
            .replace("climbStairs", "xStairs");
        let built = build()
            .draft(&draft)
            .root(root.path())
            .id(18)
            .bank(&bank)
            .call()
            .unwrap();
        let all = built.problems.join("\n");
        assert!(all.contains("solutions/javascript.js fails case"), "{all}");
    }

    #[test]
    fn schema_is_closed_and_complete() {
        let s = schema();
        let props = s["properties"].as_object().unwrap();
        assert_eq!(s["required"].as_array().unwrap().len(), props.len());
        assert_eq!(s["additionalProperties"], false);
        // The worked example in the prompt matches the schema's fields.
        let example =
            serde_json::to_value(Draft::from_question(Bank::embedded().get(1).unwrap())).unwrap();
        for key in props.keys() {
            assert!(example.get(key).is_some(), "example missing {key}");
        }
    }
}
