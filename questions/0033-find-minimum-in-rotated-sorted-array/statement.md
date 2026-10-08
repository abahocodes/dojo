A list of **distinct** integers was sorted in increasing order, then rotated:
some number of elements (possibly zero) were taken from the front and appended,
in order, to the back. For example `[1, 4, 6, 9]` could have become
`[6, 9, 1, 4]`.

Given the rotated list `nums`, return its smallest value. Aim for O(log n) time.

## Example 1

```
nums   = [6, 9, 1, 4]
output = 1
```

## Example 2

```
nums   = [-3, 0, 8, 15]
output = -3          # rotated by zero, still fully sorted
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
- All values are distinct and `nums` is an increasing list rotated by some amount.
