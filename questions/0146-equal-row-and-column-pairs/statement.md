You are given an `n x n` integer matrix `grid`. Count the pairs `(r, c)` such
that row `r` and column `c` are equal: they hold the same values in the same
order (row `r` read left to right, column `c` read top to bottom).

Every pair counts separately, so if two identical rows both match one column,
that column contributes two pairs.

## Example 1

```
grid   = [[1, 2, 1],
          [2, 5, 2],
          [1, 2, 1]]
output = 5   # rows 0 and 2 each match columns 0 and 2; row 1 matches column 1
```

## Example 2

```
grid   = [[1, 2],
          [3, 4]]
output = 0
```

## Constraints

- `n == len(grid) == len(grid[i])`
- `1 <= n <= 200`
- `1 <= grid[i][j] <= 10^5`
