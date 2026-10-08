Each domino is a pair `[a, b]` of numbers. Two dominoes are **equivalent**
if one can be rotated to match the other: `[a, b]` and `[c, d]` are
equivalent when `a == c and b == d`, or `a == d and b == c`.

Given the list `dominoes`, return how many index pairs `(i, j)` with
`i < j` hold equivalent dominoes.

## Example 1

```
dominoes = [[3, 5], [5, 3], [2, 2], [3, 5]]
output   = 3    # indices (0,1), (0,3), (1,3)
```

## Example 2

```
dominoes = [[1, 4], [4, 2], [2, 2], [6, 1]]
output   = 0
```

## Constraints

- `1 <= len(dominoes) <= 4 * 10^4`
- Every domino has exactly two values, each from `1` to `9`.
