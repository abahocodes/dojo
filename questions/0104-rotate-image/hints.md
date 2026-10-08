# Hints

## Hint 1
Work out where a single cell goes. After a clockwise quarter turn, the value at
row `r`, column `c` ends up at row `c`, column `n - 1 - r`.

## Hint 2
That move can be split into two simpler ones that are each easy to do in place:
one reflection across a diagonal and one reflection across a vertical or
horizontal line.

## Hint 3
Transpose the grid (swap `matrix[r][c]` with `matrix[c][r]` for `c > r`), then
reverse every row.
