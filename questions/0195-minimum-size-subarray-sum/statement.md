You are given a positive integer `target` and an array `nums` of positive
integers. Find the shortest contiguous subarray of `nums` whose elements add up
to **at least** `target`, and return its length. If no subarray reaches
`target` (not even the whole array), return `0`.

## Example 1

```
target = 11
nums   = [3, 1, 4, 1, 5, 9, 2]
output = 2    # [5, 9] sums to 14; no single element reaches 11
```

## Example 2

```
target = 100
nums   = [10, 20, 30]
output = 0
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^4`
- `1 <= target <= 10^9`
