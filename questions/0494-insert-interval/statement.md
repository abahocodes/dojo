You are given a list `intervals` of closed intervals `[start, end]` that do not
overlap one another and are sorted by `start`, plus one more closed interval
`new_interval`.

Insert `new_interval` into the list. Wherever it overlaps existing intervals,
merge them into one interval, so the result is again sorted by `start` and
contains no two intervals that overlap. Intervals that only **touch** at an end
point, such as `[1, 2]` and `[2, 3]`, share the point `2` and therefore must be
merged too. Intervals like `[1, 2]` and `[3, 4]` share no point and stay apart.

Return the resulting list.

## Example 1

```
intervals    = [[1, 3], [6, 9]]
new_interval = [2, 5]
output       = [[1, 5], [6, 9]]
```

## Example 2

```
intervals    = [[1, 2], [4, 5], [7, 8], [10, 12]]
new_interval = [5, 7]
output       = [[1, 2], [4, 8], [10, 12]]    # [4,5], [5,7], [7,8] all join
```

## Constraints

- `0 <= len(intervals) <= 10^4`
- `0 <= start <= end <= 10^5` for every interval, including `new_interval`
- `intervals` is sorted by `start` and no two of its intervals share a point.
