Balloons are taped to a wall. Seen from above, balloon `i` occupies the closed
horizontal range `points[i] = [x_start, x_end]`. An arrow shot straight up from
position `x` flies forever and bursts **every** balloon whose range contains
`x` (end points included).

Return the minimum number of arrows needed to burst all the balloons.

## Example 1

```
points = [[10, 16], [2, 8], [1, 6], [7, 12]]
output = 2      # one arrow at x = 6 bursts [2,8] and [1,6]; x = 11 bursts the rest
```

## Example 2

```
points = [[1, 2], [3, 4], [5, 6], [7, 8]]
output = 4      # no two balloons share a point
```

## Constraints

- `1 <= len(points) <= 10^5`
- `-2^31 <= x_start <= x_end <= 2^31 - 1`
