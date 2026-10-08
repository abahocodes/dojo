You are given an array `nums` holding each integer from `1` to `n` exactly once
(a permutation), and an integer `k` that appears in `nums`.

The **median** of a subarray is found by sorting its elements and taking the
middle one. When the subarray has even length there are two middle elements;
the median is the **smaller** (left) one. For example, the median of
`[6, 2, 9, 4]` is `4`, and the median of `[5]` is `5`.

Return how many non-empty contiguous subarrays of `nums` have median exactly
`k`.

## Example 1

```
nums   = [3, 2, 1, 4, 5]
k      = 4
output = 3
# [4], [4, 5] and [1, 4, 5] have median 4.
```

## Example 2

```
nums   = [2, 3, 1]
k      = 3
output = 1
# Only [3] itself: every longer subarray containing 3 has a smaller median.
```

## Constraints

- `1 <= n == len(nums) <= 10^5`
- `nums` is a permutation of `1..n`.
- `1 <= k <= n`
- The answer fits in a 32-bit signed integer.
