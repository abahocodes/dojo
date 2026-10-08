# Approach: 2D DP over prefixes, rolled into one row

Once the lengths agree, any partial interleaving is described by how many
letters it has taken from each string: `i` from `s1` and `j` from `s2` produce
exactly `s3[:i + j]`. Let `ok[i][j]` say whether that's possible. The last
letter `s3[i + j - 1]` came either from `s1` (then `ok[i-1][j]` must hold and
`s1[i-1]` must equal it) or from `s2` (symmetrically).

Iterating `i` in the outer loop and `j` in the inner loop, `ok[j]` still holds
row `i - 1` when we read it (the "from `s1`" case), and `ok[j - 1]` already
holds row `i` (the "from `s2`" case).

```python
def is_interleave(s1, s2, s3):
    m, n = len(s1), len(s2)
    if m + n != len(s3):
        return False
    ok = [False] * (n + 1)
    for i in range(m + 1):
        for j in range(n + 1):
            if i == 0 and j == 0:
                ok[j] = True
                continue
            c = s3[i + j - 1]
            from_s1 = i > 0 and ok[j] and s1[i - 1] == c
            from_s2 = j > 0 and ok[j - 1] and s2[j - 1] == c
            ok[j] = from_s1 or from_s2
    return ok[n]
```

**Alternative:** a BFS or memoised DFS over the `(i, j)` states, stepping to
`(i + 1, j)` or `(i, j + 1)` when the next letter matches. It visits the same
states and is easy to get right in an interview.

## Complexity

- Time: O(len(s1) × len(s2)).
- Space: O(len(s2)) with the rolled row.

## Pitfalls

- Forgetting the length check: `"a"`, `"b"`, `"ab"` plus extra letters in `s3`
  would otherwise slip through or index out of range.
- Greedy choice when both strings offer the same letter is wrong; you must keep
  both possibilities alive.
- Plain recursion without memoisation branches at every shared letter and is
  exponential on inputs like `s1 = "aaaa...a"`, `s2 = "aaaa...a"`.
- Empty strings are valid: `("", "", "")` is `true`.
