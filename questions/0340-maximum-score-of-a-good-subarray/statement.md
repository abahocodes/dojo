You are given an array of positive integers `nums` and an index `k`.

For a contiguous subarray `nums[i..j]` (inclusive), define its **score** as

```
min(nums[i], ..., nums[j]) * (j - i + 1)
```

that is, its smallest element times its length. A subarray is **good** when it
contains position `k`, i.e. `i <= k <= j`.

Return the largest score achievable by any good subarray.

## Example 1

```
nums   = [2, 5, 3, 6, 4, 1]
k      = 2
output = 12    # nums[1..4] = [5, 3, 6, 4]: min 3, length 4
```

## Example 2

```
nums   = [7, 1, 9, 9, 2]
k      = 3
output = 18    # nums[2..3] = [9, 9]: min 9, length 2
```

## Constraints

- `1 <= nums.length <= 10^5`
- `1 <= nums[i] <= 2 * 10^4`
- `0 <= k < nums.length`
- The answer fits in a 32-bit signed integer.
