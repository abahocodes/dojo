You are given a list `nums` of integers that **may contain repeated values**.
Return every distinct subset of `nums`, including the empty subset and the
whole list.

Two subsets are the same if they hold the same values the same number of
times, regardless of which positions they were taken from. With
`nums = [2, 5, 2]`, picking the first `2` or the second `2` gives the same
subset `[2]`, and it must appear only once. Return the subsets in any order,
with the values inside each subset in any order.

## Example 1

```
nums   = [2, 5, 2]
output = [[], [2], [2, 2], [2, 2, 5], [2, 5], [5]]
```

## Example 2

```
nums   = [0]
output = [[], [0]]
```

## Example 3

```
nums   = [3, 3, 3]
output = [[], [3], [3, 3], [3, 3, 3]]
```

## Constraints

- `0 <= len(nums) <= 10`
- `-10 <= nums[i] <= 10`
