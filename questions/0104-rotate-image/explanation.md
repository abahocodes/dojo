# Approach: transpose, then mirror each row

A clockwise quarter turn sends cell `(r, c)` to `(c, n - 1 - r)`. That is the
composition of two reflections, both easy to do in place:

1. **Transpose** — swap `(r, c)` with `(c, r)` — sends `(r, c)` to `(c, r)`.
2. **Reverse each row** — sends `(c, r)` to `(c, n - 1 - r)`.

```python
def rotate(matrix):
    n = len(matrix)
    for r in range(n):
        for c in range(r + 1, n):
            matrix[r][c], matrix[c][r] = matrix[c][r], matrix[r][c]
    for row in matrix:
        row.reverse()
    return matrix
```

## Alternative: rotate four cells at a time

Walk the outer ring inward. For each layer, every cell belongs to a cycle of four
cells (top → right → bottom → left) that you can rotate with one temporary
variable. Same complexity, trickier indices.

## Complexity

- Time: O(n²) — every cell is touched a constant number of times.
- Space: O(1) extra.

## Pitfalls

- Transposing with `c` running over the whole row swaps every pair twice and
  leaves the grid unchanged — start `c` at `r + 1`.
- Reversing the **row order** instead of each row gives a counter-clockwise turn.
- Building `zip(*matrix[::-1])` is correct and short, but uses O(n²) extra memory.
