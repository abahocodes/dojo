You are given a list of integers `nums`. Rearrange it so that every `0` sits at
the **end** of the list, while all the non-zero values keep the same relative
order they had originally.

Do the rearrangement in place, using only O(1) extra memory, and then return
the list.

## Example 1

```
nums   = [0, 5, 0, -2, 8]
output = [5, -2, 8, 0, 0]
```

## Example 2

```
nums   = [3, 1]
output = [3, 1]      # nothing to move
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
