You are given an array `nums` of length `n` in which every value lies in the
range `[1, n]`. Some values may appear more than once, so others may not
appear at all.

Return every value in `[1, n]` that does **not** appear in `nums`, in
increasing order.

## Example 1

```
nums   = [3, 1, 3, 5, 5]
output = [2, 4]
```

## Example 2

```
nums   = [1]
output = []
```

## Constraints

- `1 <= n == len(nums) <= 10^5`
- `1 <= nums[i] <= n`

Follow-up: can you do it with no extra space besides the output (modifying
`nums` is allowed)?
