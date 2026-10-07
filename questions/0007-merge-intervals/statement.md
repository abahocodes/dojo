You are given a list of closed intervals, each written as `[start, end]` with
`start <= end`. The list is in no particular order.

Combine every group of intervals that overlap into a single interval covering
the whole group. Two intervals overlap if they share at least one point, so
`[1, 3]` and `[3, 6]` merge into `[1, 6]`.

Return the merged intervals sorted by their start.

## Example 1

```
intervals = [[8, 10], [1, 4], [2, 5], [12, 12]]
output    = [[1, 5], [8, 10], [12, 12]]
```

## Example 2

```
intervals = [[5, 7], [1, 5]]
output    = [[1, 7]]        # touching at 5 counts as overlapping
```

## Constraints

- `1 <= len(intervals) <= 10^4`
- `0 <= start <= end <= 10^5`
