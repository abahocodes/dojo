You are given a person's calendar as a list of meetings, where
`intervals[i] = [start, end]` means a meeting occupies the half-open time
range `[start, end)`. The list is in no particular order.

The person can only be in one meeting at a time. Return `true` if they can
attend every meeting, that is, if no two meetings overlap. Because the ranges
are half-open, a meeting that ends at time `t` does **not** conflict with one
that starts at time `t`. An empty calendar can always be attended.

## Example 1

```
intervals = [[0, 30], [5, 10], [15, 20]]
output    = false
```

The meeting `[0, 30)` overlaps both of the others.

## Example 2

```
intervals = [[7, 10], [2, 4], [4, 7]]
output    = true
```

In time order the meetings are `[2,4)`, `[4,7)`, `[7,10)`: each one starts
exactly when the previous one ends.

## Constraints

- `0 <= len(intervals) <= 10^4`
- `0 <= start < end <= 10^6`
