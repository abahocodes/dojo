Given an array of non-negative integers `nums`, imagine it sorted in
ascending order. Return the largest difference between two neighbouring
elements of that sorted order. If `nums` has fewer than two elements,
return `0`.

Your solution should run in linear time (in `n`) and use linear extra space.

## Example 1

```
nums   = [9, 1, 4, 20, 16]
output = 7    # sorted: 1, 4, 9, 16, 20; gaps 3, 5, 7, 4
```

## Example 2

```
nums   = [42]
output = 0
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `0 <= nums[i] <= 10^9`
