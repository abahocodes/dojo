Given an integer array `nums` and an integer `val`, return the elements of
`nums` that are **not** equal to `val`, in the same relative order they
appear in `nums`.

Practice the in-place technique: a read pointer scans `nums` while a write
pointer marks where the next kept element goes; the answer is the prefix up
to the write pointer.

## Example 1

```
nums   = [4, 1, 4, 2, 3, 4]
val    = 4
output = [1, 2, 3]
```

## Example 2

```
nums   = [7, 7]
val    = 7
output = []
```

## Constraints

- `0 <= len(nums) <= 100`
- `0 <= nums[i] <= 100`
- `0 <= val <= 100`
