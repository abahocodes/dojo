You are given a list of intervals, each written as `[start, end]` with
`start < end`. Delete as few intervals as possible so that no two of the
remaining intervals overlap, and return how many you deleted.

Intervals that only touch at an endpoint, such as `[1, 3]` and `[3, 5]`, do
**not** overlap.

## Example 1

```
intervals = [[1, 4], [2, 3], [3, 6], [5, 8]]
output    = 2      # e.g. keep [2, 3] and [3, 6]
```

## Example 2

```
intervals = [[1, 2], [2, 3]]
output    = 0      # touching is allowed
```

## Constraints

- `1 <= len(intervals) <= 10^5`
- `-5 * 10^4 <= start < end <= 5 * 10^4`
