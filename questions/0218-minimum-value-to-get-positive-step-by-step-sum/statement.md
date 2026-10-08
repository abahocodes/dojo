Pick a positive integer `start`. Beginning with a running total equal to
`start`, add the elements of `nums` one at a time from left to right. The
choice is **valid** if the running total is at least `1` after every single
addition.

Return the smallest valid `start` (it is always at least `1`).

## Example 1

```
nums   = [-4, 2, -3, 6]
output = 6    # totals 2, 4, 1, 7; starting at 5 the third total is 0
```

## Example 2

```
nums   = [3, 1]
output = 1    # the totals only grow
```

## Constraints

- `1 <= len(nums) <= 100`
- `-100 <= nums[i] <= 100`
