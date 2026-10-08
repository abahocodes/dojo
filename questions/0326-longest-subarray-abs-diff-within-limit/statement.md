You are given a list of positive integers `nums` and a non-negative integer
`limit`.

Find the longest non-empty contiguous subarray in which every pair of elements
differs by at most `limit` (in absolute value) — equivalently, its largest
element minus its smallest element is at most `limit`. Return the length of
that subarray.

## Example 1

```
nums = [9, 4, 6, 8, 3], limit = 4
output = 3
```

`[4, 6, 8]` has spread `8 - 4 = 4`. Both subarrays of length 4, `[9, 4, 6, 8]`
and `[4, 6, 8, 3]`, have spread 5, which exceeds the limit.

## Example 2

```
nums = [5, 5, 5, 5], limit = 0
output = 4
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^9`
- `0 <= limit <= 10^9`
