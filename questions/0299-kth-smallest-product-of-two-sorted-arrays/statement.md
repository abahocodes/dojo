You are given two integer arrays `nums1` and `nums2`, each sorted in
non-decreasing order. They may contain negative numbers and zeros.

Form the product `nums1[i] * nums2[j]` for every pair of indices, giving
`len(nums1) * len(nums2)` products (equal products from different pairs are
kept separately). Return the `k`-th smallest of them, counting from 1.

## Example 1

```
nums1  = [-3, 0, 2]
nums2  = [-1, 4]
k      = 2
output = -2   # products sorted: -12, -2, 0, 0, 3, 8
```

## Example 2

```
nums1  = [1, 2, 3]
nums2  = [2, 5]
k      = 4
output = 6    # products sorted: 2, 4, 5, 6, 10, 15
```

## Constraints

- `1 <= len(nums1), len(nums2) <= 5 * 10^4`
- `-10^5 <= nums1[i], nums2[j] <= 10^5`
- Both arrays are sorted in non-decreasing order.
- `1 <= k <= len(nums1) * len(nums2)`

Products reach `10^10` in absolute value and `k` can exceed `2^31`; use 64-bit
integers.
