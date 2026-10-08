You are given a list of non-negative integers `nums`.

A **ramp** is a pair of indices `(i, j)` with `i < j` and `nums[i] <= nums[j]`;
its **width** is `j - i`. Return the largest width of any ramp in `nums`, or `0`
if there is no ramp at all.

## Example 1

```
nums = [7, 2, 4, 1, 3, 0]
output = 3
```

The ramp `(1, 4)` pairs `2` with `3`. Nothing pairs with the `7`, and no ramp
reaches further.

## Example 2

```
nums = [5, 4, 3, 2]
output = 0      # strictly decreasing: no ramp exists
```

## Constraints

- `2 <= len(nums) <= 5 * 10^4`
- `0 <= nums[i] <= 5 * 10^4`
