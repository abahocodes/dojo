You are given an integer array `nums` and an integer `target`. Find every
group of four values `[a, b, c, d]`, taken from **four different positions**
of `nums`, with `a + b + c + d == target`.

Two groups that contain the same values (as multisets, ignoring order) count
as the same group; report each distinct group once. Return the groups in any
order; the order of values inside a group does not matter either. If no group
exists, return an empty list.

## Example 1

```
nums   = [3, -1, 0, 2, -2, 1]
target = 0
output = [[-2, -1, 0, 3], [-2, -1, 1, 2]]
```

## Example 2

```
nums   = [4, 4, 4, 4, 4]
target = 16
output = [[4, 4, 4, 4]]   # many index choices, but only one distinct group
```

## Constraints

- `1 <= len(nums) <= 200`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- Sums of four values can exceed the 32-bit range: use 64-bit arithmetic.
