You are given an integer array `nums` and a non-negative integer `k`. Decide
whether some value repeats **close together**: return `true` if there are two
different positions `i` and `j` with `nums[i] == nums[j]` and
`|i - j| <= k`, otherwise `false`.

## Example 1

```
nums   = [4, 7, 2, 7, 9]
k      = 2
output = true    # the 7s at positions 1 and 3 are 2 apart
```

## Example 2

```
nums   = [5, 1, 3, 5, 1, 3]
k      = 2
output = false   # every repeated value is exactly 3 positions apart
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`
