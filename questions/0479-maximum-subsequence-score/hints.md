# Hints

## Hint 1
The minimum of `nums2` is decided by a single picked index. Try fixing which
index supplies that minimum.

## Hint 2
If index `i` supplies the minimum, every other picked index must have
`nums2 >= nums2[i]`, and among those you want the `k - 1` largest `nums1`
values. Sorting by `nums2` in decreasing order makes "the allowed indices" a
growing prefix.

## Hint 3
Walk the indices in decreasing `nums2` order while keeping the `k` largest
`nums1` values seen so far in a min-heap, along with their sum. After each
step with exactly `k` values in the heap, `sum * current nums2` is a candidate.
