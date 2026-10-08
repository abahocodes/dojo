You are given an integer array `nums` and two integers `min_k` and `max_k`.
A **fixed-bound subarray** is a non-empty contiguous subarray whose minimum
value is exactly `min_k` and whose maximum value is exactly `max_k`.

Return the number of fixed-bound subarrays. Subarrays are counted by their
start and end positions. Note that `min_k` may be greater than `max_k`, in
which case no subarray qualifies.

## Example 1

```
nums   = [2, 4, 3, 2, 6, 4]
min_k  = 2
max_k  = 4
output = 4    # [2,4] [2,4,3] [2,4,3,2] [4,3,2]
```

## Example 2

```
nums   = [3, 3, 3]
min_k  = 3
max_k  = 3
output = 6    # every subarray
```

## Constraints

- `2 <= len(nums) <= 10^5`
- `1 <= nums[i], min_k, max_k <= 10^6`
- The answer can exceed `2^31 - 1`; return it as a 64-bit integer.
