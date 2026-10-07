You are given a list of integers `nums` and an integer `target`. Exactly one pair
of **different positions** `i` and `j` holds values that add up to `target`.

Return the two positions as a list `[i, j]`. The order of the two positions does
not matter.

## Example 1

```
nums   = [3, 8, 11, 4]
target = 12
output = [1, 3]      # 8 + 4 = 12
```

## Example 2

```
nums   = [5, 5]
target = 10
output = [0, 1]      # the same value may appear twice, at different positions
```

## Constraints

- `2 <= len(nums) <= 10^4`
- `-10^9 <= nums[i], target <= 10^9`
- Exactly one valid pair exists.

**Follow-up:** can you do better than checking every pair?
