Given an array of positive integers `nums` and an integer `k`, count the
contiguous subarrays (non-empty) whose elements multiply to a value **strictly
less than** `k`.

## Example 1

```
nums   = [4, 2, 5, 1]
k      = 20
output = 8    # [4] [2] [5] [1] [4,2] [2,5] [5,1] [2,5,1]
```

## Example 2

```
nums   = [1, 2, 3]
k      = 1
output = 0    # every product is at least 1
```

## Constraints

- `1 <= len(nums) <= 3 * 10^4`
- `1 <= nums[i] <= 1000`
- `0 <= k <= 10^6`
