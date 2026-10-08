You are given an `m x n` integer matrix `grid` in which every row and every
column is sorted in **non-increasing** order (values never go up as you move
right along a row or down along a column).

Return how many entries of `grid` are strictly negative. Zero does not count
as negative.

Try to beat the obvious `O(m * n)` scan: `O(m + n)` is possible.

## Example 1

```
grid   = [[5,  3,  0, -1],
          [2,  1, -2, -3],
          [1, -1, -4, -6]]
output = 6
```

## Example 2

```
grid   = [[4, 2],
          [0, 0]]
output = 0
```

## Constraints

- `1 <= m, n <= 300`
- `-100 <= grid[i][j] <= 100`
- `grid[i][j] >= grid[i][j + 1]` and `grid[i][j] >= grid[i + 1][j]`
  wherever both entries exist.
