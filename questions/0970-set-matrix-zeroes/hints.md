# Hints

## Hint 1
Clearing rows and columns as soon as you find a zero is a trap: the new
zeros you write look exactly like original zeros to the rest of the scan.
Separate the work into two passes: first find out what to clear, then clear.

## Hint 2
A cell `(r, c)` ends up `0` exactly when row `r` or column `c` contained an
original zero. Two boolean arrays, one per row and one per column, are
enough to record that, for O(m + n) extra space.

## Hint 3
For O(1) space, store those flags inside the grid: use `matrix[r][0]` as the
flag for row `r` and `matrix[0][c]` as the flag for column `c`. Row 0 and
column 0 are themselves flag storage, so first record in two separate
booleans whether they originally held a zero, and clear them last.
