You are given an integer array `nums`. A pair of indices `(i, j)` with
`i < j` is **bad** if

```
j - i != nums[j] - nums[i]
```

Return the number of bad pairs. The answer can exceed the 32-bit range.

## Example 1

```
nums   = [5, 2, 7, 6, 9]
output = 7   # of the 10 pairs, only (0,2), (0,4) and (2,4) are good
```

## Example 2

```
nums   = [4, 5, 6, 7]
output = 0   # the values grow exactly as fast as the indices
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^9`
