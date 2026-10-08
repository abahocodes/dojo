A floor plan is a grid `rooms` of integers:

- `-1` is a wall,
- `0` is a gate,
- `2147483647` (2³¹ − 1) is an empty room.

You can move one step at a time up, down, left or right, through gates and
empty rooms but never through walls. Fill every empty room with the number of
steps to its **nearest** gate. Rooms that can't reach any gate keep the value
`2147483647`. Walls and gates are unchanged.

Return the filled grid.

## Example 1

```
INF = 2147483647
rooms = [
  [INF,  -1,   0, INF],
  [INF, INF, INF,  -1],
  [INF,  -1, INF,  -1],
  [  0,  -1, INF, INF]
]
output = [
  [3, -1, 0,  1],
  [2,  2, 1, -1],
  [1, -1, 2, -1],
  [0, -1, 3,  4]
]
```

## Example 2

```
INF = 2147483647
rooms = [
  [  0, INF, INF],
  [ -1,  -1, INF],
  [INF, INF,  -1]
]
output = [
  [         0,          1,  2],
  [        -1,         -1,  3],
  [2147483647, 2147483647, -1]   # walled off from the gate
]
```

## Constraints

- `1 <= rows, cols <= 250`
- Every cell is `-1`, `0` or `2147483647`.
