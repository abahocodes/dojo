# Approach: staircase walk from the top-right corner

Stand on the top-right cell `(r, c) = (0, n - 1)`. Its row extends only to the
left, with smaller-or-equal values, and its column extends only downward, with
larger-or-equal values. Compare it with `target`:

- equal: found;
- greater than `target`: every cell below it in column `c` is at least as
  large, so column `c` can be dropped from the search: `c -= 1`;
- less than `target`: every cell to its left in row `r` is at most as large,
  so row `r` can be dropped: `r += 1`.

Each step removes one row or one column from the remaining sub-grid, whose
top-right corner is always the current cell. When `r` passes the last row or
`c` passes the first column, nothing is left and the target is absent.

```python
def search_matrix_sorted(matrix, target):
    rows, cols = len(matrix), len(matrix[0])
    r, c = 0, cols - 1
    while r < rows and c >= 0:
        value = matrix[r][c]
        if value == target:
            return True
        if value > target:
            c -= 1
        else:
            r += 1
    return False
```

## Complexity

- Time: O(m + n): at most `m + n - 1` steps.
- Space: O(1).

## Pitfalls

- Treating the grid as one flattened sorted array, as in the simpler matrix
  search problem. Here row `i + 1` may start below the end of row `i`.
- Starting at the top-left or bottom-right corner. From there both moves
  go toward larger (or both toward smaller) values, so one comparison does not
  tell you which way to go.
- Binary searching only the first row or column to pick "the" row: the
  target may be in several candidate rows.
