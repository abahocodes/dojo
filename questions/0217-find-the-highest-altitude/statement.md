A cyclist rides along a route of `n + 1` points, numbered `0` to `n`. The ride
starts at point `0` at altitude `0`. For each `i`, `gain[i]` is the change in
altitude from point `i` to point `i + 1` (negative means downhill).

Return the highest altitude the cyclist is at on any point of the route,
including the starting point.

## Example 1

```
gain   = [-2, 4, 3, -6, 2]
output = 5    # altitudes: 0, -2, 2, 5, -1, 1
```

## Example 2

```
gain   = [-3, -1, 2]
output = 0    # altitudes: 0, -3, -4, -2; the start is the highest
```

## Constraints

- `1 <= len(gain) <= 100`
- `-100 <= gain[i] <= 100`
