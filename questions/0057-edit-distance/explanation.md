# Approach: 2-D DP over prefixes (Levenshtein distance)

Let `D[i][j]` be the fewest edits that turn `word1[:i]` into `word2[:j]`.

- Base cases: `D[i][0] = i` (delete everything) and `D[0][j] = j` (insert
  everything).
- If `word1[i-1] == word2[j-1]`, the last characters already agree:
  `D[i][j] = D[i-1][j-1]`.
- Otherwise the last edit is one of:
  - delete `word1[i-1]`: `D[i-1][j] + 1`
  - insert `word2[j-1]`: `D[i][j-1] + 1`
  - replace `word1[i-1]` with `word2[j-1]`: `D[i-1][j-1] + 1`

Each row depends only on the row above, so keep two rows:

```python
def edit_distance(word1, word2):
    m = len(word2)
    prev = list(range(m + 1))
    for i, c1 in enumerate(word1, 1):
        curr = [i] + [0] * m
        for j, c2 in enumerate(word2, 1):
            if c1 == c2:
                curr[j] = prev[j - 1]
            else:
                curr[j] = 1 + min(prev[j], curr[j - 1], prev[j - 1])
        prev = curr
    return prev[m]
```

## Complexity

- Time: O(len(word1) × len(word2)).
- Space: O(len(word2)) with rolling rows.

## Pitfalls

- The first column of each new row is `i`, not `0`: turning `i` characters
  into the empty string takes `i` deletions.
- Empty strings are allowed on either side; the answer is the other length.
- Matching characters cost nothing, so don't add `1` in the equal case.
- Plain recursion branches three ways per step and is exponential; memoise or
  use the table.
- Edit distance is symmetric, but mixing up which index belongs to which word
  in the recurrence still breaks the fill order.
