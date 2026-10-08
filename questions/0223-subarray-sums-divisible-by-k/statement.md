Given an integer array `nums` (which may contain negative numbers) and an
integer `k`, count the non-empty **contiguous** subarrays whose sum is
divisible by `k`. A sum of `0`, or a negative multiple of `k` such as `-2k`,
counts as divisible.

Subarrays at different positions are counted separately, even if they contain
the same values.

## Example 1

```
nums   = [3, -1, 4, 2, -5, 6]
k      = 4
output = 3     # [4], [3, -1, 4, 2] (sum 8) and [-1, 4, 2, -5] (sum 0)
```

## Example 2

```
nums   = [-6, 4, 2]
k      = 2
output = 6     # every element is even, so all 6 subarrays qualify
```

## Constraints

- `1 <= len(nums) <= 3 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `2 <= k <= 10^4`
