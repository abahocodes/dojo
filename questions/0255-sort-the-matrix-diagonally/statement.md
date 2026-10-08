A **diagonal** of a matrix is a maximal line of cells running from top-left
to bottom-right: it starts at a cell in the top row or the left column and
moves one row down and one column right at each step until it leaves the
matrix. For example, in a 4 x 5 matrix the diagonal starting at `(1, 0)` is
`(1, 0), (2, 1), (3, 2)`.

Given an `m x n` integer matrix `mat`, sort every diagonal in ascending order
(smallest value at its top-left end) and return the resulting matrix.

## Example 1

```
mat    = [[5, 4, 1],
          [2, 3, 7],
          [9, 1, 6]]
output = [[3, 4, 1],
          [1, 5, 7],
          [9, 2, 6]]
```

The main diagonal `5, 3, 6` becomes `3, 5, 6`, and the diagonal `2, 1`
becomes `1, 2`. The others are already sorted.

## Example 2

```
mat    = [[8, 6, 4, 2]]
output = [[8, 6, 4, 2]]   # every diagonal has a single cell
```

## Constraints

- `1 <= m, n <= 100`
- `1 <= mat[i][j] <= 100`
