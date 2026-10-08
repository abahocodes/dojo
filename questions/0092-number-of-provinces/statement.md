There are `n` towns, numbered `0` to `n - 1`. You are given an `n × n` matrix
`is_connected` where `is_connected[i][j] == 1` means a road runs directly
between town `i` and town `j`, and `0` means it doesn't. Roads work both ways,
so the matrix is symmetric, and every town counts as connected to itself.

A **province** is a largest possible group of towns in which every town can
reach every other town, directly or by passing through other towns of the
group.

Return the number of provinces.

## Example 1

```
is_connected = [
  [1, 1, 0, 0],
  [1, 1, 0, 0],
  [0, 0, 1, 1],
  [0, 0, 1, 1]
]
output = 2      # {0, 1} and {2, 3}
```

## Example 2

```
is_connected = [
  [1, 0, 0],
  [0, 1, 0],
  [0, 0, 1]
]
output = 3      # no roads: every town is its own province
```

## Constraints

- `1 <= n <= 200`
- `is_connected[i][j]` is `0` or `1`.
- `is_connected[i][i] == 1`
- `is_connected[i][j] == is_connected[j][i]`
