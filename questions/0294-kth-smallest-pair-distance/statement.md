You are given an integer array `nums`. For every pair of positions `i < j`,
the pair's **gap** is `|nums[i] - nums[j]|`. There are `n * (n - 1) / 2` such
pairs.

Sort all the gaps in non-decreasing order (equal gaps from different pairs are
listed separately) and return the `k`-th one, counting from 1.

## Example 1

```
nums   = [1, 6, 3]
k      = 2
output = 3   # gaps are 5, 2, 3 -> sorted 2, 3, 5
```

## Example 2

```
nums   = [4, 4, 9, 4]
k      = 4
output = 5   # three pairs of 4s have gap 0; the other three pairs have gap 5
```

## Constraints

- `2 <= n == len(nums) <= 10^4`
- `0 <= nums[i] <= 10^6`
- `1 <= k <= n * (n - 1) / 2`
