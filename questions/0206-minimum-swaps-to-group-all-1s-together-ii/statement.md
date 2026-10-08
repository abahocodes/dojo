You are given a binary array `nums` (every element is `0` or `1`) that is
**circular**: the last element is considered adjacent to the first.

A swap exchanges the values stored at any two positions; the positions do not
have to be adjacent. Return the minimum number of swaps needed so that all the
`1`s occupy one contiguous block of the circle. The block may wrap around from
the end of the array to the start.

If the array has no `1`s, or the `1`s are already contiguous, the answer is `0`.

## Example 1

```
nums   = [1, 0, 0, 1, 0, 1, 1, 0]
output = 1    # swap positions 0 and 4: [0, 0, 0, 1, 1, 1, 1, 0]
```

## Example 2

```
nums   = [1, 1, 0, 0, 1]
output = 0    # the 1s at 4, 0, 1 already form a block that wraps around
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `nums[i]` is `0` or `1`
