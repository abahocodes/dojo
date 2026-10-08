You are given a rectangular grid of integers `matrix` with two ordering guarantees:

- each row is sorted in strictly increasing order from left to right, and
- the first value of every row is larger than the last value of the row above it.

Return `true` if `target` appears somewhere in the grid, and `false` otherwise.

## Example 1

```
matrix = [
  [ 2,  5,  8, 11],
  [14, 17, 20, 23],
  [30, 34, 41, 50]
]
target = 20
output = true
```

## Example 2

```
matrix = [
  [ 2,  5,  8, 11],
  [14, 17, 20, 23],
  [30, 34, 41, 50]
]
target = 13
output = false     # 13 would sit between 11 and 14
```

## Constraints

- `1 <= rows, cols <= 100`
- `-10^4 <= matrix[r][c], target <= 10^4`
- Both ordering guarantees above hold.

**Follow-up:** can you answer in O(log(rows * cols)) time?
