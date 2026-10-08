You are given an array `nums` that contains only `0`s and `1`s. Find the
longest **contiguous** subarray that contains as many `0`s as `1`s, and return
its length. If no such subarray exists, return `0`.

## Example 1

```
nums   = [1, 0, 0, 1, 1, 0, 1]
output = 6     # [1, 0, 0, 1, 1, 0] (or [0, 0, 1, 1, 0, 1]) has three of each
```

## Example 2

```
nums   = [1, 1, 1]
output = 0     # there is no 0 to balance any 1
```

## Constraints

- `1 <= len(nums) <= 10^5`
- every `nums[i]` is `0` or `1`
