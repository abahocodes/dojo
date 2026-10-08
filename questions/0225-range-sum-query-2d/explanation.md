# Approach: 2-D prefix sums (inclusion-exclusion)

Let `pre[i][j]` be the sum of every cell in rows `0..i-1` and columns
`0..j-1`; row `0` and column `0` of the table are zeros. The table is built in
one pass: the rectangle ending at `(i, j)` is the cell itself plus the
rectangle above plus the rectangle to the left, minus their overlap, which was
counted twice.

A query rectangle is the big rectangle ending at `(r2, c2)`, minus the strip
above row `r1`, minus the strip left of column `c1`, plus the corner that both
strips removed.

```python
def region_sums(matrix, queries):
    m, n = len(matrix), len(matrix[0])
    pre = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(m):
        for j in range(n):
            pre[i + 1][j + 1] = matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j]
    return [
        pre[r2 + 1][c2 + 1] - pre[r1][c2 + 1] - pre[r2 + 1][c1] + pre[r1][c1]
        for r1, c1, r2, c2 in queries
    ]
```

Every table entry has absolute value at most `200 * 200 * 10^4 = 4 * 10^8`, so
32-bit ints are enough.

## Complexity

- Time: O(m * n + q) for `q` queries.
- Space: O(m * n) for the table.

## Pitfalls

- Off-by-one errors between matrix indices and table indices. The padding row
  and column mean `pre` is always indexed one past the matrix.
- Forgetting to add back the corner `pre[r1][c1]`, which both strips removed.
- Mixing up the query order: it is `[r1, c1, r2, c2]`, rows and columns
  interleaved, not `[r1, r2, c1, c2]`.
