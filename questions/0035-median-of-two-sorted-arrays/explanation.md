# Approach: binary search on the partition of the shorter list

Let `m <= n` (swap if needed) and `half = (m + n + 1) // 2`. Choosing `i`
elements from `nums1` forces `j = half - i` elements from `nums2` into the left
half of the merged list. The partition is valid when every left element is `<=`
every right element, which reduces to two cross checks:

- `left1 = nums1[i-1] <= right2 = nums2[j]`
- `left2 = nums2[j-1] <= right1 = nums1[i]`

If `left1 > right2`, we took too many from `nums1` (move left); otherwise if
`left2 > right1` we took too few (move right). Out-of-range neighbours count as
∓infinity.

```python
def find_median_sorted_arrays(nums1, nums2):
    if len(nums1) > len(nums2):
        nums1, nums2 = nums2, nums1
    m, n = len(nums1), len(nums2)
    half = (m + n + 1) // 2
    lo, hi = 0, m
    inf = float("inf")
    while True:
        i = (lo + hi) // 2
        j = half - i
        left1 = nums1[i - 1] if i > 0 else -inf
        right1 = nums1[i] if i < m else inf
        left2 = nums2[j - 1] if j > 0 else -inf
        right2 = nums2[j] if j < n else inf
        if left1 > right2:
            hi = i - 1
        elif left2 > right1:
            lo = i + 1
        else:
            if (m + n) % 2:
                return float(max(left1, left2))
            return (max(left1, left2) + min(right1, right2)) / 2
```

## Complexity

- Time: O(log(min(m, n))) — binary search over the shorter list only.
- Space: O(1).

## Pitfalls

- Search on the shorter list; otherwise `j` can go negative or past the end.
- Sentinels: when `i == 0` there is no left element in `nums1`, so use
  -infinity; when `i == m`, use +infinity on the right (same for `j`).
- With `half = (m + n + 1) // 2` the left half holds the extra element for odd
  totals, so the median is simply the largest left value.
- Return a float even when the median is a whole number.
