You are given an integer array `nums`, whose values may be negative, and a
positive integer `k`.

Find the shortest non-empty block of consecutive elements of `nums` whose
total is **at least** `k`, and return its length. If no such block exists,
return `-1`.

## Example 1

```
nums   = [3, -2, 4, -1, 5]
k      = 7
output = 3    # [4, -1, 5] sums to 8; no block of length 1 or 2 reaches 7
```

## Example 2

```
nums   = [1, -1, 2]
k      = 4
output = -1   # the best total of any block is 2
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^5 <= nums[i] <= 10^5`
- `1 <= k <= 10^9`
