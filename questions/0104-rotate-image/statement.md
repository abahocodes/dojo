You are given an `n x n` grid of integers `matrix`, such as the pixels of a square
image. Turn it a quarter turn **clockwise** and return the result.

Only the returned grid is checked, but try to do the rotation **in place** —
rearranging the values inside `matrix` itself with O(1) extra memory — and then
return `matrix`.

## Example 1

```
matrix = [
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
]
output = [
  [7, 4, 1],
  [8, 5, 2],
  [9, 6, 3]
]
```

## Example 2

```
matrix = [
  [10, 20],
  [30, 40]
]
output = [
  [30, 10],
  [40, 20]
]
```

## Constraints

- `1 <= n <= 20`
- `len(matrix[r]) == n` for every row
- `-1000 <= matrix[r][c] <= 1000`
