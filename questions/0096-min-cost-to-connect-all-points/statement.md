You are given `points`, a list of distinct integer coordinates `[x, y]` on a
plane. Linking two points `[x1, y1]` and `[x2, y2]` with a cable costs their
**Manhattan distance**, `|x1 - x2| + |y1 - y2|`.

Lay cables so that every point can reach every other point through cables
(directly or via other points). Return the minimum total cost.

## Example 1

```
points = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]
output = 20
# [0,0]-[2,2] costs 4, [2,2]-[5,2] costs 3, [5,2]-[7,0] costs 4,
# [2,2]-[3,10] costs 9: 4 + 3 + 4 + 9 = 20
```

## Example 2

```
points = [[3, 12], [-2, 5], [-4, 1]]
output = 18      # [3,12]-[-2,5] costs 12, [-2,5]-[-4,1] costs 6
```

## Constraints

- `1 <= len(points) <= 1000`
- `-10^6 <= x, y <= 10^6`
- All points are distinct.
