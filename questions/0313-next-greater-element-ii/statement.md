You are given an integer array `nums` arranged in a circle: after the last
element comes the first one again.

For each index `i`, start just after `i` and walk forward around the circle
(wrapping from the end back to the start). The answer for `i` is the first
value you meet that is **strictly greater** than `nums[i]`. If you get all the
way back to `i` without meeting one, the answer is `-1`.

Return the array of answers, one per index, in index order.

## Example 1

```
nums   = [2, 7, 3, 5]
output = [7, -1, 5, 7]   # 5 wraps around past 2 to reach 7
```

## Example 2

```
nums   = [4, 4, 1]
output = [-1, -1, 4]     # equal values do not count as greater
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
