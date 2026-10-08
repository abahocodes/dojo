You are given two lists of the same length `n`, `nums1` and `nums2`, and an
integer `k`.

Pick exactly `k` distinct indices. The **score** of your pick is

```
(sum of nums1[i] over the picked i) * (minimum of nums2[i] over the picked i)
```

Return the largest score you can achieve.

## Example 1

```
nums1  = [2, 4, 1, 5]
nums2  = [3, 1, 6, 2]
k      = 2
output = 14
```

Indices 0 and 3 score `(2 + 5) * min(3, 2) = 14`. Indices 0 and 2 have the
larger minimum (3) but only score `(2 + 1) * 3 = 9`, and indices 2 and 3 score
`(1 + 5) * 2 = 12`.

## Example 2

```
nums1  = [7, 3, 9]
nums2  = [1, 8, 2]
k      = 1
output = 24
```

Choosing only index 1 scores `3 * 8 = 24`.

## Constraints

- `1 <= k <= n <= 10^5`
- `0 <= nums1[i], nums2[i] <= 10^5`
- The answer can exceed the 32-bit range.
