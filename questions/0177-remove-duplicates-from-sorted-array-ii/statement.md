You are given an integer array `nums` sorted in non-decreasing order. Thin it
out so that **each value appears at most twice**: if a value occurs three or
more times, keep only its first two occurrences and drop the rest. Values that
occur once or twice are kept as they are.

Return the resulting array; its relative order is the same as in `nums`.
Aim for one pass with O(1) extra space besides the output (a write pointer
that overwrites the array in place is the classic solution).

## Example 1

```
nums   = [1, 1, 1, 2, 2, 2, 2, 3]
output = [1, 1, 2, 2, 3]
```

## Example 2

```
nums   = [-5, 0, 0, 0, 7]
output = [-5, 0, 0, 7]
```

## Constraints

- `1 <= len(nums) <= 3 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is sorted in non-decreasing order.
