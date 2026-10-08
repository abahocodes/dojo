You are given an integer array `nums` and an integer `k`. Let `M` be the
largest value in the whole array.

Count the contiguous, non-empty subarrays of `nums` in which `M` occurs **at
least** `k` times, and return that count.

## Example 1

```
nums   = [2, 5, 1, 5, 5, 3]
k      = 2
output = 10   # e.g. [5, 1, 5], [5, 5], [2, 5, 1, 5, 5, 3], ...
```

## Example 2

```
nums   = [4, 4, 1]
k      = 3
output = 0    # 4 appears only twice in the whole array
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^6`
- `1 <= k <= 10^5`
- The answer can exceed `2^31 - 1`; return it as a 64-bit integer.
