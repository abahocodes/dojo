You are given an integer array `nums` sorted in non-decreasing order; it may
contain negative numbers. Return an array of the squares of its elements,
also sorted in non-decreasing order.

Squaring and then sorting works, but try to do it in O(n) time.

## Example 1

```
nums   = [-5, -2, 0, 3, 4]
output = [0, 4, 9, 16, 25]
```

## Example 2

```
nums   = [-6, -1, 1, 2]
output = [1, 1, 4, 36]
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is sorted in non-decreasing order.
