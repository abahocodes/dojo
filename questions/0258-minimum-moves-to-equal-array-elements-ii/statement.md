You are given an integer array `nums`. In one move you may add 1 to, or
subtract 1 from, any single element.

Return the minimum number of moves needed to make all elements equal. The
answer can exceed the 32-bit range, so return it as a 64-bit integer.

## Example 1

```
nums   = [1, 10, 2, 9]
output = 16   # e.g. make everything 5: 4 + 5 + 3 + 4
```

## Example 2

```
nums   = [-3, 7, 7]
output = 10   # make everything 7
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
