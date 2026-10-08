You are given a rectangular grid of integers `matrix`. Read it as a clockwise
spiral: start at the top-left cell, go right along the top row, down the right
column, left along the bottom row, up the left column, and keep circling inward
until every cell has been visited exactly once.

Return the values in the order you read them.

## Example 1

```
matrix = [
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
]
output = [1, 2, 3, 6, 9, 8, 7, 4, 5]
```

## Example 2

```
matrix = [
  [ 3,  1,  4,  1],
  [ 5,  9,  2,  6],
  [-5, -3,  5,  8]
]
output = [3, 1, 4, 1, 6, 8, 5, -3, -5, 5, 9, 2]
```

## Constraints

- `1 <= rows, cols <= 10`
- `-100 <= matrix[r][c] <= 100`
