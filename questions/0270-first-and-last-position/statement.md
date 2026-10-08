`nums` is sorted in non-decreasing order, so all copies of any value sit next
to each other in one contiguous block. Given a value `target`, find that block.

Return a two-element array `[first, last]`: the smallest and the largest index
`i` with `nums[i] == target`. If `target` does not appear in `nums`, return
`[-1, -1]`.

Your solution must run in `O(log n)` time, even when `target` occurs many
times.

## Example 1

```
nums   = [1, 3, 3, 3, 6, 8, 8]
target = 3
output = [1, 3]
```

## Example 2

```
nums   = [2, 4, 7, 9]
target = 5
output = [-1, -1]   # 5 would go between 4 and 7, but it is not there
```

## Constraints

- `0 <= len(nums) <= 10^5`
- `-10^9 <= nums[i], target <= 10^9`
- `nums` is sorted in non-decreasing order
