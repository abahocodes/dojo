You are given two integer lists `nums1` and `nums2`, each sorted in
non-decreasing order, and an integer `k`.

Every index pair `(i, j)` (with `i` indexing `nums1` and `j` indexing `nums2`)
forms the value pair `[nums1[i], nums2[j]]` with sum `nums1[i] + nums2[j]`.
Line up all `len(nums1) · len(nums2)` index pairs in this order:

1. by sum, smallest first;
2. among equal sums, by smaller `i` first;
3. among equal sums and equal `i`, by smaller `j` first.

Return the first `k` pairs of that line-up, in order, each as
`[nums1[i], nums2[j]]`. If there are fewer than `k` pairs in total, return all
of them.

## Example 1

```
nums1  = [1, 3, 8]
nums2  = [2, 5, 9]
k      = 4
output = [[1, 2], [3, 2], [1, 5], [3, 5]]
```

The four smallest sums are 3, 5, 6 and 8.

## Example 2

```
nums1  = [0, 0]
nums2  = [1]
k      = 5
output = [[0, 1], [0, 1]]
```

Only two pairs exist. Both have sum 1; the one with `i = 0` comes first, but
they look identical.

## Constraints

- `1 <= len(nums1), len(nums2) <= 10^4`
- `-10^9 <= nums1[i], nums2[j] <= 10^9`
- `nums1` and `nums2` are sorted in non-decreasing order.
- `1 <= k <= 10^4`
