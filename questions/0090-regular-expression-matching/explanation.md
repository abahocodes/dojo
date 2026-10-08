# Approach: DP over suffixes of the string and the pattern

Let `dp[i][j]` be true when the suffix `s[i:]` is matched by the suffix
`p[j:]`. Two empty suffixes match: `dp[len(s)][len(p)] = true`, and a
non-empty string never matches an empty pattern.

For `j < len(p)`, let `first` be whether the pattern element `p[j]` matches the
character `s[i]` (it requires `i < len(s)` and `p[j]` equal to `s[i]` or `.`):

- If `p[j + 1] == '*'`, the element `p[j]*` either matches zero characters
  (`dp[i][j + 2]`) or eats one character and stays put (`first and dp[i + 1][j]`).
- Otherwise `p[j]` must match exactly one character: `first and dp[i + 1][j + 1]`.

Filling `i` from `len(s)` down to `0` and `j` from `len(p) - 1` down to `0`
makes every referenced cell available. The answer is `dp[0][0]`.

```python
def is_match(s, p):
    m, n = len(s), len(p)
    dp = [[False] * (n + 1) for _ in range(m + 1)]
    dp[m][n] = True
    for i in range(m, -1, -1):
        for j in range(n - 1, -1, -1):
            first = i < m and p[j] in (s[i], ".")
            if j + 1 < n and p[j + 1] == "*":
                dp[i][j] = dp[i][j + 2] or (first and dp[i + 1][j])
            else:
                dp[i][j] = first and dp[i + 1][j + 1]
    return dp[0][0]
```

Note the row `i = len(s)` matters: an empty string can still be matched by
patterns like `a*b*.*`.

## Complexity

- Time: O(len(s) × len(p)).
- Space: O(len(s) × len(p)); keeping only two rows brings it to O(len(p)).

## Pitfalls

- Plain backtracking explodes on inputs like `s = "aaaa...a"` and
  `p = "a*a*a*...a*b"`; memoise or use the table.
- `*` applies to the element **before** it, so always look one character ahead
  in the pattern rather than handling `*` when you reach it.
- The match must cover the whole string: `"ab"` does not match `"a"`.
- An empty string can match a non-empty pattern (`"c*"`), so don't return
  `false` as soon as `s` runs out.
