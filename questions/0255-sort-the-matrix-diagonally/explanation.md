# Approach: sort each diagonal independently

Cells `(i, j)` with the same `i - j` lie on the same diagonal, and diagonals
never share cells, so each one can be sorted on its own. Every diagonal starts
either in the left column `(i, 0)` or in the top row `(0, j)`. For each start,
walk down-right collecting values, sort them, then walk again writing them
back.

```python
def diagonal_sort(mat):
    m, n = len(mat), len(mat[0])
    res = [row[:] for row in mat]
    starts = [(i, 0) for i in range(m)] + [(0, j) for j in range(1, n)]
    for si, sj in starts:
        length = min(m - si, n - sj)
        values = sorted(res[si + k][sj + k] for k in range(length))
        for k in range(length):
            res[si + k][sj + k] = values[k]
    return res
```

## Complexity

- Time: O(m * n * log(min(m, n))): every cell is sorted once inside a
  diagonal of length at most `min(m, n)`. A counting sort over the value range
  `1..100` makes it O(m * n + (m + n) * 100).
- Space: O(min(m, n)) for one diagonal's values, plus the output copy.

## Pitfalls

- Sorting anti-diagonals (top-right to bottom-left) instead. The key is
  `i - j`, not `i + j`.
- Starting a walk at `(0, 0)` twice, once from the row and once from the
  column. Harmless here, but wasted work.
- Assuming the matrix is square: diagonal length is `min(m - i, n - j)`.
