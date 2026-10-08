A rectangular plot of land is divided into an `m x n` grid of unit squares.
`height_map[r][c]` is the ground height of the square in row `r`, column `c`.
It rains heavily, filling every pocket of the plot with water. Water flows
between squares that share an edge (not diagonally), and any water that
reaches a square on the outer border of the grid drains off the plot, so a
border square never holds water.

Return the total volume of water that stays on the plot once it stops
raining, where one unit of water fills a `1 x 1` square to a depth of `1`.

## Example 1

```
height_map = [[3, 3, 3, 3],
              [3, 0, 1, 3],
              [3, 3, 3, 3]]
output     = 5
```

The two inner squares are enclosed by walls of height 3, so they fill to
level 3: `3 + 2 = 5` units.

## Example 2

```
height_map = [[5, 5, 5, 5, 5],
              [5, 1, 1, 1, 5],
              [5, 1, 4, 1, 5],
              [5, 1, 1, 1, 2],
              [5, 5, 5, 5, 5]]
output     = 8
```

The low border square of height 2 lets the water inside drain down to level
2. The eight squares of height 1 each hold one unit; the square of height 4
holds none.

## Constraints

- `1 <= m, n <= 200`
- `0 <= height_map[r][c] <= 2 * 10^4`
