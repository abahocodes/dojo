A frame of width `k` slides across the integer list `nums`, starting over
the first `k` elements and moving one position to the right at a time until it
covers the last `k` elements.

Return a list with the largest value inside the frame at each of its
`len(nums) - k + 1` positions, in order.

## Example 1

```
nums   = [4, 2, 12, 3, 8, 7, 1]
k      = 3
output = [12, 12, 12, 8, 8]
```

## Example 2

```
nums   = [9, -2]
k      = 1
output = [9, -2]     # a frame of width 1 just copies the list
```

## Constraints

- `1 <= k <= len(nums) <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

**Follow-up:** can you make the total work O(n), independent of `k`?
