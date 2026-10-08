# Approach: 2-D DP over prefixes, one row at a time

Let `L[i][j]` be the LCS length of `a[:i]` and `b[:j]`, with `L[0][*] =
L[*][0] = 0`.

- If `a[i-1] == b[j-1]`, matching them is never worse than not matching
  them: `L[i][j] = L[i-1][j-1] + 1`.
- Otherwise at least one of them is not in the common subsequence:
  `L[i][j] = max(L[i-1][j], L[i][j-1])`.

Row `i` depends only on row `i - 1`, so keep two rows of length
`len(b) + 1`:

```python
def longest_common_subsequence(a, b):
    prev = [0] * (len(b) + 1)
    for ca in a:
        curr = [0] * (len(b) + 1)
        for j, cb in enumerate(b, 1):
            if ca == cb:
                curr[j] = prev[j - 1] + 1
            else:
                curr[j] = max(prev[j], curr[j - 1])
        prev = curr
    return prev[-1]
```

## Complexity

- Time: O(len(a) × len(b)).
- Space: O(len(b)) with rolling rows (O(len(a) × len(b)) for the full table).

## Pitfalls

- Subsequences are not substrings: characters need not be contiguous.
- Recursion with memoisation works but recurses up to `len(a) + len(b)` deep;
  an iterative table avoids stack limits.
- When rolling rows, read the diagonal `prev[j - 1]` from the previous row,
  not the one you're currently writing.
- Greedily matching the first common character can be wrong, e.g. `"ab"`
  vs. `"ba"` style inputs; the DP considers both options.
