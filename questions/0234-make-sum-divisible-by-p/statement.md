You are given an array of positive integers `nums` and a positive integer
`p`. You may delete one contiguous subarray of `nums` (it may be empty, but
you may **not** delete the entire array) so that the sum of the elements left
over is divisible by `p`.

Return the length of the shortest subarray you can delete to achieve this,
or `-1` if it cannot be done.

## Example 1

```
nums   = [5, 3, 8, 2]
p      = 7
output = 2   # total 18; deleting [3, 8] leaves 7
```

## Example 2

```
nums   = [2, 3]
p      = 10
output = -1  # only deleting everything would leave a multiple of 10
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^9`
- `1 <= p <= 10^9`
