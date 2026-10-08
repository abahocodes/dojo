A rectangular wall is built from rows of bricks. All bricks are one unit
tall, and `wall[r]` lists the widths of the bricks in row `r` from left to
right. Every row has the same total width.

You want to draw one vertical line from the top of the wall to the bottom.
The line may not run along the left or right outer edge of the wall. Where
the line passes through the seam between two adjacent bricks of a row, it
does not cross either of them; otherwise it crosses exactly one brick of
that row.

Return the smallest possible number of bricks the line crosses.

## Example 1

```
wall   = [[2, 1, 3], [1, 2, 3], [3, 3], [2, 4], [1, 1, 1, 3]]
output = 1   # a line 3 units from the left only crosses the [2, 4] row
```

## Example 2

```
wall   = [[4], [4], [4]]
output = 3   # no interior seams, so every row is crossed
```

## Constraints

- `1 <= len(wall) <= 10^4`
- `1 <= len(wall[r]) <= 10^4`
- The total number of bricks is at most `2 * 10^4`.
- `1 <= wall[r][c] <= 2^31 - 1`
- Every row has the same total width, which is at most `2^31 - 1`.
