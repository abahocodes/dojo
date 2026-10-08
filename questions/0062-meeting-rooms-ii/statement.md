You are given a list of meetings, each written as `[start, end]`. A meeting
occupies a room from time `start` up to, but **not including**, time `end`, so
a meeting ending at `10` and another starting at `10` can use the same room
one after the other.

Return the smallest number of rooms needed to hold every meeting.

## Example 1

```
intervals = [[0, 30], [5, 10], [15, 20]]
output    = 2      # [0, 30] needs its own room; the other two share one
```

## Example 2

```
intervals = [[9, 12], [12, 15], [2, 6]]
output    = 1      # back-to-back meetings may reuse the room
```

## Constraints

- `1 <= len(intervals) <= 10^4`
- `0 <= start < end <= 10^6`
