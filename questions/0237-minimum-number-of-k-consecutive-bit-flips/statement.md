You are given a binary array `nums` (every element is `0` or `1`) and an
integer `k`. A **k-flip** picks a contiguous block of exactly `k` elements and
inverts each of them: every `0` in the block becomes `1` and every `1` becomes
`0`.

Return the fewest k-flips needed to turn every element of `nums` into `1`. If
no sequence of k-flips can do it, return `-1`.

## Example 1

```
nums   = [0, 0, 0, 1, 0, 1, 1, 0]
k      = 3
output = 3    # flip [0..2] -> [1, 1, 1, 1, 0, 1, 1, 0]
              # flip [4..6] -> [1, 1, 1, 1, 1, 0, 0, 0]
              # flip [5..7] -> [1, 1, 1, 1, 1, 1, 1, 1]
```

## Example 2

```
nums   = [1, 1, 0]
k      = 2
output = -1   # the only blocks are [0..1] and [1..2]; no combination of
              # them fixes the last element without breaking another
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `nums[i]` is `0` or `1`
