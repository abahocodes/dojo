Given two integer arrays `nums1` and `nums2`, return their intersection **with
multiplicity**: a value that occurs `a` times in `nums1` and `b` times in
`nums2` appears `min(a, b)` times in the result.

You may return the values in any order.

## Example 1

```
nums1  = [4, 1, 4, 2]
nums2  = [4, 4, 4, 3, 1]
output = [4, 4, 1]   # 4 appears min(2, 3) = 2 times, 1 appears once
```

## Example 2

```
nums1  = [5, 6]
nums2  = [7, 8]
output = []
```

## Constraints

- `1 <= len(nums1), len(nums2) <= 10^4`
- `0 <= nums1[i], nums2[i] <= 1000`
