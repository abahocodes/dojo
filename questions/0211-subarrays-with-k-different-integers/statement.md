You are given an integer array `nums` and an integer `k`. Count the
contiguous, non-empty subarrays of `nums` that contain **exactly** `k`
distinct values, and return that count.

Subarrays at different positions are counted separately even if they hold the
same values.

## Example 1

```
nums   = [1, 2, 1, 3, 2]
k      = 2
output = 5    # [1,2], [2,1], [1,3], [3,2], [1,2,1]
```

## Example 2

```
nums   = [3, 3, 3]
k      = 1
output = 6    # every one of the 6 subarrays holds only the value 3
```

## Constraints

- `1 <= len(nums) <= 2 * 10^4`
- `1 <= nums[i] <= len(nums)`
- `1 <= k <= len(nums)`
