You are given two integer arrays `nums1` and `nums2`, each already sorted in
non-decreasing order. Return one array, also in non-decreasing order, that
contains every element of both inputs (duplicates included).

Practice the in-place technique: imagine `nums1` had `len(nums2)` empty
slots at its end, and fill the result from the **back**, always placing the
larger of the two remaining tail elements. That way no element is
overwritten before it has been moved.

## Example 1

```
nums1  = [1, 4, 7]
nums2  = [2, 4, 9, 10]
output = [1, 2, 4, 4, 7, 9, 10]
```

## Example 2

```
nums1  = []
nums2  = [-3, 5]
output = [-3, 5]
```

## Constraints

- `0 <= len(nums1), len(nums2) <= 200`
- `1 <= len(nums1) + len(nums2)`
- `-10^9 <= nums1[i], nums2[j] <= 10^9`
- Both arrays are sorted in non-decreasing order.
