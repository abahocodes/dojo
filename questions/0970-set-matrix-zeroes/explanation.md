# Approach: use the first row and column as markers

Cell `(r, c)` must become `0` exactly when row `r` or column `c` held an
original zero. The straightforward version records that in two boolean
arrays `zero_row[m]` and `zero_col[n]` in a first pass, then clears in a
second pass: O(m * n) time, O(m + n) space.

To get **O(1) extra space**, keep those flags inside the grid itself:

- `matrix[r][0]` is the flag for row `r`,
- `matrix[0][c]` is the flag for column `c`.

Writing a `0` there is harmless: that cell is going to be cleared anyway,
because its row (or column) has a zero.

The catch is that row 0 and column 0 are now doing two jobs. Before using
them as flags, record in two booleans whether row 0 and column 0 originally
contained a zero. Then:

1. Scan the inner cells `(r >= 1, c >= 1)`; for each zero, set
   `matrix[r][0] = 0` and `matrix[0][c] = 0`.
2. Scan the inner cells again and clear `(r, c)` if its row flag or column
   flag is `0`.
3. Finally clear row 0 and/or column 0 using the two booleans.

Row 0 and column 0 must be cleared **last**: clearing them earlier would wipe
out the flags step 2 reads.

```python
def set_zeroes(matrix):
    rows, cols = len(matrix), len(matrix[0])
    first_row_zero = any(matrix[0][c] == 0 for c in range(cols))
    first_col_zero = any(matrix[r][0] == 0 for r in range(rows))

    for r in range(1, rows):
        for c in range(1, cols):
            if matrix[r][c] == 0:
                matrix[r][0] = 0
                matrix[0][c] = 0

    for r in range(1, rows):
        for c in range(1, cols):
            if matrix[r][0] == 0 or matrix[0][c] == 0:
                matrix[r][c] = 0

    if first_row_zero:
        for c in range(cols):
            matrix[0][c] = 0
    if first_col_zero:
        for r in range(rows):
            matrix[r][0] = 0
    return matrix
```

## Complexity

- Time: O(m * n), a constant number of passes over the grid.
- Space: O(1) extra (two booleans); the grid is updated in place.

## Pitfalls

- Clearing while scanning: the zeros you write get mistaken for original
  zeros and spread across the whole grid.
- Using a sentinel value (like `-1` or "infinity") to mark cells to clear:
  every 32-bit value can legitimately appear in the grid, so no value is safe.
- Using `matrix[0][0]` for both row 0 and column 0: it can only hold one
  flag. That is why row 0 and column 0 get their own booleans here.
- Clearing row 0 or column 0 before step 2, which destroys the markers.
- One row or one column grids: the inner loops simply don't run and the two
  booleans do all the work.
