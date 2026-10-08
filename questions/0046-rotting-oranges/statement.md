A crate of oranges is laid out as a `grid` of cells. Each cell holds:

- `0`: nothing,
- `1`: a fresh orange,
- `2`: a rotten orange.

Every minute, each fresh orange that shares a side (up, down, left or right)
with a rotten orange becomes rotten itself. All of these changes happen at the
same time.

Return the number of minutes until no fresh orange is left. If some fresh
orange can never rot, return `-1`. If there are no fresh oranges to begin
with, the answer is `0`.

## Example 1

```
grid = [
  [1, 1, 0, 2],
  [0, 1, 1, 1],
  [0, 0, 1, 1]
]
output = 5      # the orange at the top-left corner is the last to rot
```

## Example 2

```
grid = [
  [2, 0, 1],
  [1, 0, 1]
]
output = -1     # the empty middle column shields the right side
```

## Constraints

- `1 <= rows, cols <= 100`
- Every cell is `0`, `1` or `2`.
