Given an integer array `arr`, remove exactly one contiguous subarray (it may
be empty) so that the elements left over, kept in their original order, form
a **non-decreasing** sequence.

Return the minimum possible length of the removed subarray. Removing nothing
(length 0) is allowed when `arr` is already non-decreasing, and the remaining
array may be empty.

## Example 1

```
arr    = [1, 5, 6, 2, 3, 7]
output = 2    # remove [5, 6] -> [1, 2, 3, 7]  (or [2, 3] -> [1, 5, 6, 7])
```

## Example 2

```
arr    = [4, 3, 2, 1]
output = 3    # only one element can stay
```

## Constraints

- `1 <= len(arr) <= 10^5`
- `0 <= arr[i] <= 10^9`
