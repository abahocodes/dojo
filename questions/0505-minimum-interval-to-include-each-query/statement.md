You are given a list of closed integer ranges `intervals`, where
`intervals[i] = [left, right]` covers every integer `x` with
`left <= x <= right`. The **size** of a range is the number of integers it
covers: `right - left + 1`.

You are also given an array `queries`. For each query value `q`, find the
smallest size among all ranges that contain `q`. If no range contains `q`, the
answer for that query is `-1`.

Return the answers in the same order as `queries`.

## Example 1

```
intervals = [[2, 5], [3, 3], [1, 8], [6, 7]]
queries   = [3, 1, 6, 9, 5]
output    = [1, 8, 2, -1, 4]
```

`3` is inside `[3, 3]` (size 1). Only `[1, 8]` (size 8) covers `1`. `6` is in
`[1, 8]` and `[6, 7]`; the smaller has size 2. Nothing covers `9`. `5` is in
`[2, 5]` (size 4) and `[1, 8]`.

## Example 2

```
intervals = [[10, 20], [12, 14]]
queries   = [14, 14, 21, 10]
output    = [3, 3, -1, 11]
```

Queries may repeat, and a range contains both of its endpoints.

## Constraints

- `1 <= len(intervals) <= 10^5`
- `1 <= len(queries) <= 10^5`
- `intervals[i].length == 2`
- `1 <= left <= right <= 10^7`
- `1 <= queries[j] <= 10^7`
