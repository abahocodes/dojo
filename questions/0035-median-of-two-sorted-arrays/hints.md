# Hints

## Hint 1
Merging the lists takes O(m + n). The median only depends on splitting the
merged list into a left half and a right half. Can you describe that split
without building the merged list?

## Hint 2
Take `i` elements from the front of `nums1` and `j = (m + n + 1) // 2 - i` from
the front of `nums2`. The split is correct exactly when
`nums1[i-1] <= nums2[j]` and `nums2[j-1] <= nums1[i]`.

## Hint 3
Binary-search `i` over `[0, m]` on the shorter list. If `nums1[i-1] > nums2[j]`,
`i` is too large; if `nums2[j-1] > nums1[i]`, `i` is too small. Treat
out-of-range neighbours as -infinity / +infinity. Once valid, the median is
`max(left sides)` for odd totals, or the average of `max(left sides)` and
`min(right sides)` for even totals.
