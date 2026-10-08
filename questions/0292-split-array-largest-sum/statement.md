You are given an array of non-negative integers `nums` and an integer `k`.
Cut `nums` into exactly `k` contiguous pieces, each holding at least one
element. Every element belongs to exactly one piece, and the pieces keep the
original order.

The **load** of a cut is the largest sum among its `k` pieces. Return the
smallest load any cut can achieve.

## Example 1

```
nums   = [4, 1, 6, 3, 2]
k      = 2
output = 11   # [4, 1, 6] | [3, 2] has sums 11 and 5; no cut does better
```

## Example 2

```
nums   = [1, 4, 4]
k      = 3
output = 4    # every element is its own piece
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `0 <= nums[i] <= 10^5`
- `1 <= k <= min(50, len(nums))`
