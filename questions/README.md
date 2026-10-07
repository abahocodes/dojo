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
- Every supported language has a `languages` entry. Supported today:
  `python` (`.py`) and `javascript` (`.js`).
- The signature is language-agnostic. `function` and param names are
  snake_case; JavaScript uses camelCase (`two_sum` → `twoSum`).
- Types: `int`, `float`, `bool`, `string`, `ListNode`, `TreeNode`, plus `[]`
  suffixes. In tests, `ListNode` is an array and `TreeNode` a level-order array
  with nulls. Harnesses provide `ListNode`/`TreeNode` to solutions.
- `compare`: `exact` (default), `unordered`, `unordered_deep`, `float`.
- At least 2 visible and 3 hidden cases. Hidden cases run on `/submit` only.
- Questions are not versioned: edit them in place. Attempts record the dojo
  release they ran on, which pins the exact copy of every question.
- Statements must be original wording — never copied from other sites.

## Tools

```
dojo scaffold questions/NNNN-slug    # draft boilerplate from the signature, register it in meta.json
dojo validate                        # organization, schema, tests, reference solutions, boilerplate
cargo test                           # the same checks, one test per question (asset_0001_two_sum, ...)
dojo schema                          # regenerate schema/*.json after changing the types
```

`dojo validate` runs every reference solution against every case and checks
that every boilerplate loads, defines the function and does not already pass.
It needs `python3` and `node` installed.
