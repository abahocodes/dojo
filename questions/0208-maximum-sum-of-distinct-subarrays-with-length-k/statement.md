You are given an integer array `nums` and a window length `k`. Look at every
contiguous subarray of `nums` that has exactly `k` elements **and** in which
no value repeats.

Return the largest sum among those subarrays. If no length-`k` subarray has
all-distinct elements, return `0`.

## Example 1

```
nums   = [4, 2, 2, 7, 1, 3]
k      = 3
output = 11   # [7, 1, 3]; [4, 2, 2] and [2, 2, 7] repeat a value
```

## Example 2

```
nums   = [6, 6, 6]
k      = 2
output = 0    # every window of length 2 is [6, 6]
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^5`
- The answer can exceed `2^31 - 1`; return it as a 64-bit integer.
