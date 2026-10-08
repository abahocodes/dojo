# Hints

## Hint 1
There can be `10^8` pairs, so generating and sorting all of them is out. Only
about `k` pairs ever need to be looked at closely.

## Hint 2
Fix `i`. Because `nums2` is sorted, the pairs `(i, 0), (i, 1), (i, 2), ...`
already come in the required order. You have up to `len(nums1)` sorted
sequences to merge, and you only need the first `k` merged items.

## Hint 3
Seed a min-heap with `(nums1[i] + nums2[0], i, 0)` for each `i < min(k, len(nums1))`.
Pop `k` times; after popping `(sum, i, j)`, push `(nums1[i] + nums2[j + 1], i, j + 1)`
if it exists. Comparing heap entries by `(sum, i, j)` gives the tie-break for
free. Sums can reach `2 · 10^9`, so use 64-bit integers for them.
