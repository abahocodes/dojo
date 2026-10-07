Given a list of integers `nums`, build a list `out` of the same length where
`out[i]` is the product of every element of `nums` **except** `nums[i]`.

You may **not** use division, and your solution should run in O(n) time.

## Example 1

```
nums   = [2, 3, 5, 4]
output = [60, 40, 24, 30]
```

## Example 2

```
nums   = [4, 0, -2, 1]
output = [0, -8, 0, 0]
```

## Constraints

- `2 <= len(nums) <= 10^5`
- `-30 <= nums[i] <= 30`
- Every prefix and suffix product fits in a 32-bit signed integer.

**Follow-up:** can you use only O(1) extra space besides the output list?
