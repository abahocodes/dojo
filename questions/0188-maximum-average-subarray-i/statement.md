You are given an integer array `nums` and an integer `k`. Consider every
contiguous block of **exactly** `k` consecutive elements of `nums`. Return the
largest average (sum divided by `k`) among those blocks.

Answers within `1e-6` of the exact value are accepted.

## Example 1

```
nums   = [4, -2, 9, 3, -6, 8]
k      = 3
output = 3.66667    # block [4, -2, 9] has the largest sum, 11; 11 / 3 = 3.66666...
```

## Example 2

```
nums   = [-7]
k      = 1
output = -7.0
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
