Given an integer array `nums` (values may be negative) and an integer `k`,
return the length of the **longest** contiguous subarray whose elements add up
to exactly `k`. If no subarray sums to `k`, return `0`.

## Example 1

```
nums   = [2, -3, 5, 1, -2, 4]
k      = 3
output = 5     # [2, -3, 5, 1, -2] sums to 3
```

## Example 2

```
nums   = [-1, 4, 2]
k      = 9
output = 0     # no subarray reaches 9
```

## Constraints

- `1 <= len(nums) <= 2 * 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `-10^9 <= k <= 10^9`
