You are given an `m x n` integer matrix `matrix` and a list of rectangle
queries. Each query is `[r1, c1, r2, c2]`, with `r1 <= r2` and `c1 <= c2`, and
names the rectangle whose top-left cell is `(r1, c1)` and whose bottom-right
cell is `(r2, c2)` (rows and columns are 0-indexed, both corners included).

Return an array whose `q`-th element is the sum of all cells inside the `q`-th
rectangle, in the same order as the queries. Aim to answer each query in
constant time after some preprocessing.

## Example 1

```
matrix  = [[3, 0, 1, 4],
           [5, 6, 3, 2],
           [1, 2, 0, 1]]
queries = [[0, 0, 1, 1], [1, 1, 2, 3], [2, 0, 2, 0]]
output  = [14, 14, 1]
# 3+0+5+6 = 14;  6+3+2+2+0+1 = 14;  the single cell (2, 0) holds 1
```

## Example 2

```
matrix  = [[-2, 7],
           [4, -1]]
queries = [[0, 0, 1, 1], [0, 1, 1, 1]]
output  = [8, 6]
```

## Constraints

- `1 <= m, n <= 200`
- `-10^4 <= matrix[i][j] <= 10^4`
- `1 <= len(queries) <= 10^4`
- `0 <= r1 <= r2 < m` and `0 <= c1 <= c2 < n` for every query
