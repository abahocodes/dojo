You are given an integer array `nums` and an integer `k`. In one operation
you pick two elements of the array (at different positions) whose sum is
exactly `k` and remove both of them.

Return the largest number of operations you can perform.

## Example 1

```
nums   = [3, 1, 4, 2, 2, 5]
k      = 6
output = 2   # remove 1 and 5, then 4 and 2; [3, 2] is left
```

## Example 2

```
nums   = [2, 2, 2, 3]
k      = 4
output = 1   # one pair of 2s; the third 2 has no partner left
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^9`
- `1 <= k <= 10^9`
