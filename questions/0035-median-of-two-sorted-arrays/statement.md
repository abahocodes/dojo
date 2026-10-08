You are given two lists of integers, `nums1` and `nums2`, each sorted in
non-decreasing order (either may be empty, but not both). Imagine merging them
into one sorted list.

Return the median of that merged list: the middle value when its length is odd,
or the average of the two middle values when its length is even. Aim for
O(log(min(m, n))) time, where `m` and `n` are the two lengths.

## Example 1

```
nums1  = [2, 7]
nums2  = [4]
output = 4.0         # merged: [2, 4, 7]
```

## Example 2

```
nums1  = [1, 3, 8]
nums2  = [5, 9, 10]
output = 6.5         # merged: [1, 3, 5, 8, 9, 10], (5 + 8) / 2
```

## Constraints

- `0 <= len(nums1), len(nums2) <= 1000`
- `1 <= len(nums1) + len(nums2)`
- `-10^6 <= nums1[i], nums2[i] <= 10^6`
- Answers within `1e-6` of the exact median are accepted.
