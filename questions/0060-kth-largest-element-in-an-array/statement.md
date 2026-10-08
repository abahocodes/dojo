You are given a list of integers `nums` and an integer `k`. Imagine lining the
values up from largest to smallest; return the value standing in position `k`
(counting from 1).

Equal values each take their own position: in `[5, 5, 1]` the 1st **and** the
2nd largest are both `5`.

## Example 1

```
nums   = [3, 9, 4, 9, 1, 7]
k      = 3
output = 7      # largest to smallest: 9, 9, 7, 4, 3, 1
```

## Example 2

```
nums   = [-2]
k      = 1
output = -2
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

**Follow-up:** sorting everything is O(n log n). Can you avoid sorting the
whole list?
