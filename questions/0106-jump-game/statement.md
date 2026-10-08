You stand on index 0 of a list of non-negative integers `nums`. From index `i` you
may jump forward by any distance from 1 up to `nums[i]` (a value of 0 means you
are stuck there).

Return `true` if some sequence of jumps lands you on the **last index**, and
`false` otherwise.

## Example 1

```
nums   = [3, 1, 0, 2, 4]
output = true      # 0 -> 3 -> 4
```

## Example 2

```
nums   = [2, 1, 0, 3]
output = false     # every route ends up on index 2, which has value 0
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `0 <= nums[i] <= 10^5`
