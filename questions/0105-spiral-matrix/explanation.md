# Approach: shrinking boundaries

Keep the unvisited part of the grid as a rectangle bounded by `top`, `bottom`,
`left` and `right`. Each lap reads its four sides in order and pulls the matching
boundary inward. When the rectangle collapses to one row or one column, the last
two sides would re-read cells, so guard them.

```python
def spiral_order(matrix):
    top, bottom = 0, len(matrix) - 1
    left, right = 0, len(matrix[0]) - 1
    out = []
    while top <= bottom and left <= right:
        for c in range(left, right + 1):
            out.append(matrix[top][c])
        top += 1
        for r in range(top, bottom + 1):
            out.append(matrix[r][right])
        right -= 1
        if top <= bottom:
            for c in range(right, left - 1, -1):
                out.append(matrix[bottom][c])
            bottom -= 1
        if left <= right:
            for r in range(bottom, top - 1, -1):
                out.append(matrix[r][left])
            left += 1
    return out
```

## Alternative: walk and turn

Walk with a direction vector, turning clockwise whenever the next cell is outside
the grid or already visited. It's simple to reason about but needs a `visited`
grid (or a sentinel value written into the input).

## Complexity

- Time: O(rows * cols) — every cell is read once.
- Space: O(1) besides the output.

## Pitfalls

- Missing the `top <= bottom` / `left <= right` guards duplicates cells on single
  rows, single columns and odd-sized centres.
- Off-by-one in the reverse ranges: the bottom row is read from `right` down to
  `left` inclusive, the left column from `bottom` up to `top` inclusive.
- Non-square grids are where most bugs hide — test `1 x n`, `n x 1` and `2 x n`.
