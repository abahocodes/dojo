You are given a list of distinct half-open intervals `[a, b)`. Interval
`[a, b)` is **covered** by a different interval `[c, d)` when `c <= a` and
`b <= d`, that is, when it lies entirely inside it.

Remove every interval that is covered by some other interval in the list and
return how many intervals remain.

## Example 1

```
intervals = [[2, 6], [3, 5], [5, 9]]
output    = 2      # [3, 5) lies inside [2, 6)
```

## Example 2

```
intervals = [[1, 4], [1, 7], [2, 3], [6, 8]]
output    = 2      # [1, 4) and [2, 3) lie inside [1, 7)
```

## Constraints

- `1 <= len(intervals) <= 1000`
- `0 <= a < b <= 10^5` for every interval `[a, b]`
- All intervals are distinct; the list is in no particular order.
