Given a grid of integers `matrix` and an integer `target`, count the
non-empty rectangular submatrices whose elements add up to exactly `target`.

A submatrix is any block of cells `matrix[r][c]` with `r1 <= r <= r2` and
`c1 <= c <= c2`. Two submatrices are different if their corner coordinates
`(r1, c1, r2, c2)` differ, even when they contain the same values.

## Example 1

```
matrix = [[1, -1],
          [-1, 1]]
target = 0
output = 5   # two 1x2 rows, two 2x1 columns and the whole 2x2 grid
```

## Example 2

```
matrix = [[2, 0, 2]]
target = 2
output = 4   # [2] twice, [2, 0] and [0, 2]
```

## Constraints

- `1 <= len(matrix) <= 100`
- `1 <= len(matrix[0]) <= 100`, and every row has the same length
- `-1000 <= matrix[i][j] <= 1000`
- `-10^8 <= target <= 10^8`
