# Approach: 2-D prefix sums with clamped corners

Every answer is the sum of a rectangle of `mat`, so a 2-D prefix-sum table
answers each one in O(1). Build `pre`, where `pre[i][j]` sums rows `0..i-1` and
columns `0..j-1`:

`pre[i+1][j+1] = mat[i][j] + pre[i][j+1] + pre[i+1][j] - pre[i][j]`.

For cell `(i, j)` the block covers rows `r1 = max(0, i - k)` up to (but not
including) `r2 = min(m, i + k + 1)`, and columns `c1 = max(0, j - k)` up to
`c2 = min(n, j + k + 1)`. Working with those half-open bounds the block sum is
`pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1]`.

```python
def matrix_block_sum(mat, k):
    m, n = len(mat), len(mat[0])
    pre = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(m):
        for j in range(n):
            pre[i + 1][j + 1] = mat[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j]
    result = [[0] * n for _ in range(m)]
    for i in range(m):
        r1, r2 = max(0, i - k), min(m, i + k + 1)
        for j in range(n):
            c1, c2 = max(0, j - k), min(n, j + k + 1)
            result[i][j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1]
    return result
```

The largest possible sum is `100 * 100 * 100 = 10^6`.

## Complexity

- Time: O(m * n): building the table and filling the answer are both linear.
- Space: O(m * n) for the table (plus the output).

## Pitfalls

- Not clamping the block to the matrix, which indexes outside the table.
- Mixing inclusive and exclusive bounds. Pick one convention (here: half-open
  `[r1, r2)`) and apply it to both rows and columns.
- `k` can exceed the matrix size; clamping handles it, and every cell then
  holds the total of the whole matrix.
