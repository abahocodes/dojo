An island is mapped as a grid `heights`, where `heights[r][c]` is the
elevation of the cell in row `r` and column `c`. The **Pacific** ocean lies
along the top and left edges of the island; the **Atlantic** ocean lies along
the bottom and right edges.

When rain falls on a cell, the water can run to any side-adjacent cell (up,
down, left or right) whose elevation is **less than or equal to** the current
one, and keep going from there. Water on a cell next to an ocean edge can
drain straight into that ocean.

Return every cell `[r, c]` from which rainwater can reach **both** oceans. The
cells may be listed in any order.

## Example 1

```
heights = [
  [1, 2, 3],
  [8, 9, 4],
  [7, 6, 5]
]
output = [[0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]
```

The corner `[0, 2]` touches both oceans. Water at `[0, 0]` can only reach the
Pacific: both neighbours are higher.

## Example 2

```
heights = [[5]]
output  = [[0, 0]]      # a single cell borders all four edges
```

## Constraints

- `1 <= rows, cols <= 200`
- `0 <= heights[r][c] <= 100000`
