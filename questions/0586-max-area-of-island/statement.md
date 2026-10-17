A satellite survey gives you a map `grid` of `0`s (water) and `1`s (land).
Land cells that touch along an edge (up, down, left or right) belong to the
same island; touching only at a corner does not connect them. Everything
outside the map is water.

The **area** of an island is the number of land cells in it. Return the area
of the biggest island on the map, or `0` if there is no land at all.

## Example 1

```
grid   = [[0, 1, 1, 0, 0, 0],
          [0, 1, 0, 0, 1, 1],
          [1, 0, 0, 0, 1, 0],
          [1, 1, 0, 1, 1, 0],
          [0, 0, 0, 0, 0, 1]]
output = 5
```

There are four islands, of areas 3 (top left), 5 (the hook on the right),
3 (bottom left) and 1 (the lone cell in the corner). The largest has 5 cells.

## Example 2

```
grid   = [[0, 0, 0],
          [0, 0, 0]]
output = 0
```

No land, so the answer is `0`.

## Constraints

- `1 <= len(grid), len(grid[0]) <= 50`
- All rows have the same length.
- Every `grid[r][c]` is `0` or `1`.
