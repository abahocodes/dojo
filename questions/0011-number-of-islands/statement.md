A map is drawn on a rectangular `grid` where every cell is either `"1"` (land)
or `"0"` (water). Two land cells belong to the same **island** when you can walk
from one to the other through land cells, moving only up, down, left or right
(diagonal steps don't count). Everything outside the grid is water.

Return how many separate islands the map contains.

## Example 1

```
grid = [
  ["1", "1", "0", "0", "1"],
  ["0", "1", "0", "1", "1"],
  ["0", "0", "0", "0", "0"],
  ["1", "0", "1", "1", "0"]
]
output = 4
```

## Example 2

```
grid = [
  ["1", "0", "1"],
  ["0", "1", "0"],
  ["1", "0", "1"]
]
output = 5      # diagonal neighbours are not connected
```

## Constraints

- `1 <= rows, cols <= 200`
- Every cell is `"0"` or `"1"`.
