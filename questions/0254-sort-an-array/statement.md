Given an integer array `nums`, return a new array holding the same values
(duplicates included) in ascending order.

Do it **without** calling your language's built-in sorting routine, and in
`O(n log n)` time. Merge sort and heap sort are both good choices.

## Example 1

```
nums   = [7, -2, 9, 0, 3]
output = [-2, 0, 3, 7, 9]
```

## Example 2

```
nums   = [4, 1, 4, 1, 1]
output = [1, 1, 1, 4, 4]
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `-5 * 10^4 <= nums[i] <= 5 * 10^4`
