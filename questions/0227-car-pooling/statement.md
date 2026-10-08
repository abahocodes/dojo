A car drives along a straight road, always heading east, and never turns
back. It has `capacity` empty seats for passengers.

Each trip is `[passengers, start, end]`: a group of `passengers` people is
picked up at kilometre `start` and dropped off at kilometre `end`. A group
occupies its seats from `start` up to, but not including, `end`, so a group
leaving at kilometre `x` frees its seats before a group boarding at `x` needs
them.

Return `true` if the car can serve every trip without ever carrying more than
`capacity` passengers at once, otherwise `false`.

## Example 1

```
trips    = [[3, 1, 5], [2, 4, 8]]
capacity = 4
output   = false   # between km 4 and km 5 the car carries 3 + 2 = 5
```

## Example 2

```
trips    = [[3, 1, 5], [2, 5, 8]]
capacity = 3
output   = true    # the first group leaves at km 5 just as the second boards
```

## Constraints

- `1 <= len(trips) <= 1000`
- `1 <= passengers <= 100`
- `0 <= start < end <= 1000`
- `1 <= capacity <= 10^5`
