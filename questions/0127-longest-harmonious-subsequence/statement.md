Call a sequence of integers **harmonious** when its largest element is
exactly 1 more than its smallest element.

Given an integer array `nums`, return the length of its longest harmonious
**subsequence**: elements chosen from `nums` in their original order, not
necessarily next to each other. Return `0` if no harmonious subsequence
exists.

## Example 1

```
nums   = [4, 2, 3, 3, 5, 3, 2, 4]
output = 5       # [4, 3, 3, 3, 4] (or [2, 3, 3, 3, 2])
```

## Example 2

```
nums   = [7, 7, 7]
output = 0       # max - min is 0, never 1
```

## Constraints

- `1 <= len(nums) <= 2 * 10^4`
- `-10^9 <= nums[i] <= 10^9`
