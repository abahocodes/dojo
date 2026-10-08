# Approach: grid DP with one rolling row

Let `P[r][c]` be the number of routes to cell `(r, c)`. The last move into
`(r, c)` comes from above or from the left, and those two groups of routes are
disjoint:

```
P[0][c] = P[r][0] = 1
P[r][c] = P[r-1][c] + P[r][c-1]
```

When processing row `r` left to right, `row[c]` still holds `P[r-1][c]` before
it is updated, and `row[c-1]` already holds `P[r][c-1]`, so one array suffices:

```python
def unique_paths(m, n):
    row = [1] * n
    for _ in range(m - 1):
        for c in range(1, n):
            row[c] += row[c - 1]
    return row[-1]
```

**Alternative:** every route is a sequence of `m - 1` downs and `n - 1`
rights, so the answer is the binomial coefficient `C(m + n - 2, m - 1)`. That's
O(min(m, n)) time, but computing it with intermediate products can overflow
fixed-width or floating-point numbers; the DP only adds values that never
exceed the final answer.

## Complexity

- Time: O(m × n).
- Space: O(n).

## Pitfalls

- A grid with one row or one column has exactly `1` route, including the
  `1 × 1` grid (the robot is already there).
- Recursion without memoisation explores every route individually and is
  hopeless for a `29 × 29` grid (over 10^16 calls).
- Computing `C(m + n - 2, m - 1)` via factorials overflows; in JavaScript,
  floating-point division can also introduce rounding errors.
