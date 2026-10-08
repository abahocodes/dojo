You are given an array `nums` containing only `0`s and `1`s, and an integer
`k`. You may turn **at most** `k` of the zeros into ones (you may also turn
fewer, or none).

Return the length of the longest contiguous block of ones the array can
contain after your changes.

## Example 1

```
nums   = [1, 1, 0, 0, 1, 1, 1, 0, 1]
k      = 1
output = 5    # turn the 0 at index 7 into a 1: indices 4..8 are all ones
```

## Example 2

```
nums   = [0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1]
k      = 2
output = 7    # turn the zeros at indices 4 and 7: indices 4..10 are all ones
```

## Constraints

- `1 <= len(nums) <= 10^5`
- every `nums[i]` is `0` or `1`
- `0 <= k <= len(nums)`
