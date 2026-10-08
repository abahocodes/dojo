You are given an integer array `nums` sorted in non-decreasing order. Remove
the repeated values so that each distinct value appears once, keeping the
values in their original (sorted) order, and return the result.

Practice the in-place technique: a *read* pointer scans the array while a
*write* pointer marks where the next new value goes. Each value seen for the
first time is copied to the write position. The answer is the prefix of
`nums` up to the write pointer.

## Example 1

```
nums   = [1, 1, 2, 5, 5, 5, 8]
output = [1, 2, 5, 8]
```

## Example 2

```
nums   = [-4, -4, -4]
output = [-4]
```

## Constraints

- `1 <= len(nums) <= 3 * 10^4`
- `-100 <= nums[i] <= 100`
- `nums` is sorted in non-decreasing order.
