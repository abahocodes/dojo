# Hints

## Hint 1
Read the grid row by row, left to right. Thanks to the second guarantee, what
does that sequence of values look like?

## Hint 2
It is one long sorted list of `rows * cols` values, so you can binary search it
without building it. How do you turn a position `i` in that list into a row and a
column?

## Hint 3
Position `i` lives at row `i // cols`, column `i % cols`. Binary search `lo = 0`,
`hi = rows * cols - 1`, comparing `matrix[mid // cols][mid % cols]` with `target`.
