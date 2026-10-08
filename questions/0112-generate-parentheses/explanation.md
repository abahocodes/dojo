# Approach: backtracking with open/close counts

A prefix can still be completed into a balanced string as long as it never
closes more than it has opened, and never opens more than `n`. So from any
prefix we may add `(` if `open < n`, and `)` if `close < open`. Following both
choices recursively visits exactly the balanced strings, each once.

```python
def generate_parenthesis(n):
    result, current = [], []

    def backtrack(open_, close):
        if len(current) == 2 * n:
            result.append("".join(current))
            return
        if open_ < n:
            current.append("(")
            backtrack(open_ + 1, close)
            current.pop()
        if close < open_:
            current.append(")")
            backtrack(open_, close + 1)
            current.pop()

    backtrack(0, 0)
    return result
```

The recursion depth is only `2n`, so recursion is safe here.

## Complexity

- Time: O(4^n / sqrt(n)): the number of results is the n-th Catalan number,
  and each costs O(n) to build.
- Space: O(n) for the recursion and current prefix, plus the output.

## Pitfalls

- Allowing `)` whenever `close < n` produces strings like `")("`.
- Appending the shared `current` list instead of a joined copy.
- Brute force over all `2^(2n)` strings passes small inputs but explores far
  more strings than it keeps.
