You are given a non-empty list of integers `nums`. Among all **contiguous,
non-empty** stretches of `nums`, find the one whose elements have the largest
total, and return that total.

## Example 1

```
nums   = [-3, 4, -1, 2, 1, -5, 4]
output = 6      # 4 + (-1) + 2 + 1
```

## Example 2

```
nums   = [-7, -2, -9]
output = -2     # the stretch must be non-empty, so pick the least negative value
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

**Follow-up:** solve it in a single pass with O(1) extra space.
