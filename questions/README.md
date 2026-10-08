# Questions

Each question is a self-contained asset folder. dojo embeds this directory at
build time and only ever reads it.

`meta.json` is the manifest: it describes the question and names every other
file in the folder, so file names carry no meaning. The convention used here:

```
NNNN-slug/
  meta.json            (schema/meta.schema.json)
  statement.md         the problem, in original wording
  hints.md             "## Hint 1", "## Hint 2", ... from gentle to near-solution
  explanation.md       approach, code, complexity, pitfalls
  tests.json           (schema/tests.schema.json)
  boilerplate/python.py, boilerplate/javascript.js   what the user starts with
  solutions/python.py,   solutions/javascript.js     reference solutions
```

```json
{
  "$schema": "../../schema/meta.schema.json",
  "id": 1, "slug": "two-sum", "title": "Two Sum", "difficulty": "easy",
  "tags": ["arrays", "hash-map"], "companies": ["amazon"], "target_minutes": 15,
  "signature": { "function": "two_sum", "params": [...], "returns": "int[]" },
  "compare": "unordered",
  "statement": "statement.md",
  "hints": "hints.md",
  "explanation": "explanation.md",
  "tests": "tests.json",
  "languages": {
    "python":     { "boilerplate": "boilerplate/python.py",     "solution": "solutions/python.py" },
    "javascript": { "boilerplate": "boilerplate/javascript.js", "solution": "solutions/javascript.js" }
  }
}
```

## Rules

- The folder holds exactly what `meta.json` references (plus `meta.json`):
  no missing files, no stray files, no file referenced twice, no paths
  outside the folder.
- Languages: `python` (`.py`), `javascript` (`.js`), `typescript` (`.ts`),
  `java` (`.java`), `cpp` (`.cpp`) and `go` (`.go`). Python and JavaScript
  are required; the others are being added to every question.
  `dojo scaffold <folder> --lang typescript,java,cpp,go` writes starter code
  from the signature and registers the files.
- Solutions look like LeetCode's: plain functions in Python, JavaScript,
  TypeScript and Go; `class Solution { ... }` in Java and C++ (with
  `java.util.*` / the standard library and `using namespace std;` provided).
- The signature is language-agnostic. `function` and param names are
  snake_case; every language but Python uses camelCase (`two_sum` →
  `twoSum`).
- Types: `int`, `long`, `float`, `bool`, `string`, `ListNode`, `TreeNode`,
  plus `[]` suffixes. `int` is 32-bit (Java/C++ `int`); use `long` for values
  beyond ±2^31 (up to ±2^53, which JavaScript represents exactly). Node values
  are `int`s. In tests, `ListNode` is an array and `TreeNode` a level-order array
  with nulls. Harnesses provide `ListNode`/`TreeNode` to solutions.
- `compare`: `exact` (default), `unordered`, `unordered_deep`, `float`.
- At least 2 visible and 3 hidden cases. Hidden cases run on `/submit` only.
- Limits that keep questions safe to run: at most 100 cases and 200 KB of
  tests; visible inputs under 1,000 characters (they're printed); each
  reference solution under 1s per case (users get 3s) and deterministic
  (two runs, same results).
- Questions are not versioned: edit them in place. Attempts record the dojo
  release they ran on, which pins the exact copy of every question.
- Statements must be original wording — never copied from other sites.

## Tools

```
dojo scaffold questions/NNNN-slug    # draft boilerplate from the signature, register it in meta.json
dojo validate                        # organization, schema, tests, reference solutions, boilerplate
dojo validate --question NNNN-slug   # just one question (what CI runs on question PRs)
cargo test --features question-tests # the same checks, as one cargo test per question
dojo schema                          # regenerate schema/*.json after changing the types
```

`dojo validate` runs every reference solution against every case and checks
that every boilerplate loads, defines the function and does not already pass.
It needs `python3` and `node` installed.

The easiest way to add a question is `dojo contribute`: describe it, review
the draft dojo builds and validates, then `/accept` to open the PR.
