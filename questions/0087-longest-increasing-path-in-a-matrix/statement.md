A hiker stands on a rectangular height map `matrix`, where `matrix[r][c]` is
the elevation of cell `(r, c)`. From a cell they may step to a neighbouring
cell up, down, left or right (never diagonally, never off the map), but only
if that cell is **strictly higher** than the one they're on.

The hiker can start anywhere. Return the number of cells on the longest route
they can walk.

## Example 1

```
matrix = [
  [7, 8, 9],
  [6, 1, 2],
  [5, 4, 3]
]
output = 9      # 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 spirals around
```

## Example 2

```
matrix = [
  [4, 4],
  [4, 4]
]
output = 1      # every neighbour is equal, so no step is allowed
```

## Constraints

- `1 <= rows, cols <= 200`
- `0 <= matrix[r][c] <= 2^31 - 1`
