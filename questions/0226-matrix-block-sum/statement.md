Given an `m x n` matrix `mat` and an integer `k`, build a matrix `answer` of
the same shape. `answer[i][j]` is the sum of every `mat[r][c]` with

- `i - k <= r <= i + k`,
- `j - k <= c <= j + k`, and
- `(r, c)` inside the matrix.

In other words, each cell is replaced by the sum of the square block of
side `2k + 1` centred on it, clipped at the matrix borders.

## Example 1

```
mat    = [[1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]]
k      = 1
output = [[12, 21, 16],
          [27, 45, 33],
          [24, 39, 28]]
# the corner (0, 0) sees only 1+2+4+5 = 12; the centre sees all nine cells
```

## Example 2

```
mat    = [[2, 1, 4, 3]]
k      = 1
output = [[3, 7, 8, 7]]
```

## Constraints

- `1 <= m, n, k <= 100`
- `1 <= mat[i][j] <= 100`
