You are given an integer array `nums` and a window length `k`. Slide a window
of exactly `k` consecutive elements across `nums`, from the leftmost position
to the rightmost, one step at a time. For every position of the window,
report the **median** of the `k` numbers inside it.

The median of a window is its middle value once the window is sorted. When `k`
is even there are two middle values, and the median is their average (which
may end in `.5`).

Return the medians as floating-point numbers, in window order: the first entry
belongs to `nums[0..k-1]`, the next to `nums[1..k]`, and so on, giving
`len(nums) - k + 1` values. Answers within `1e-6` of the expected values are
accepted.

## Example 1

```
nums   = [5, 1, 4, 2, 8, 3, 7]
k      = 3
output = [4.0, 2.0, 4.0, 3.0, 7.0]
# [5,1,4] -> 4, [1,4,2] -> 2, [4,2,8] -> 4, [2,8,3] -> 3, [8,3,7] -> 7
```

## Example 2

```
nums   = [6, -2, 9, 4, 0]
k      = 4
output = [5.0, 2.0]
# [6,-2,9,4] sorted is [-2,4,6,9]: (4 + 6) / 2 = 5
# [-2,9,4,0] sorted is [-2,0,4,9]: (0 + 4) / 2 = 2
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
