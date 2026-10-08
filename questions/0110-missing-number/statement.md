Take all the whole numbers from `0` to `n` (that is `n + 1` numbers), remove
exactly one of them, and shuffle the rest. You get the resulting list `nums`
of length `n`. Return the number that was removed.

Try to use only O(1) extra memory.

## Example 1

```
nums   = [3, 0, 1]
output = 2        # n = 3, so the full range is 0..3
```

## Example 2

```
nums   = [0, 1]
output = 2        # the removed number can be n itself
```

## Constraints

- `1 <= n == len(nums) <= 10^4`
- `0 <= nums[i] <= n`
- All values in `nums` are distinct.
