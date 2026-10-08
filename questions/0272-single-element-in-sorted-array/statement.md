`nums` is sorted in non-decreasing order. Every value in it appears exactly
**twice**, except for one value that appears exactly **once**. Return that
lonely value.

Your solution must run in `O(log n)` time and use `O(1)` extra space.

## Example 1

```
nums   = [1, 1, 4, 6, 6, 9, 9]
output = 4
```

## Example 2

```
nums   = [2, 2, 3, 3, 8]
output = 8
```

## Constraints

- `1 <= len(nums) <= 10^5`, and `len(nums)` is odd
- `0 <= nums[i] <= 10^5`
- `nums` is sorted in non-decreasing order
- exactly one value occurs once; every other value occurs exactly twice
