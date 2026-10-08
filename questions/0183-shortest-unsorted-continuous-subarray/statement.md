Given an integer array `nums`, find the shortest contiguous subarray with this
property: if you sort just that subarray in non-decreasing order, the whole
array becomes sorted in non-decreasing order.

Return the length of that subarray. If the array is already sorted, return
`0`.

## Example 1

```
nums   = [1, 3, 2, 4]
output = 2    # sorting [3, 2] gives [1, 2, 3, 4]
```

## Example 2

```
nums   = [1, 2, 2, 3]
output = 0    # already sorted; equal neighbours are fine
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-10^5 <= nums[i] <= 10^5`
