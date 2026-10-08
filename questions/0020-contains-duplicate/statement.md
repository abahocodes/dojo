You are given a list of integers `nums`. Decide whether any value shows up
**more than once** in it.

Return `true` if at least one value is repeated, and `false` if every value in
the list is unique.

## Example 1

```
nums   = [4, 9, 2, 9, 7]
output = true        # 9 appears at positions 1 and 3
```

## Example 2

```
nums   = [10, -3, 6, 0]
output = false       # all four values are different
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

**Follow-up:** comparing every pair is O(n²). Can you answer in a single pass?
