# Hints

## Hint 1
Split the current range at `mid`. In a rotated sorted array, at least one of
the halves `[lo, mid]` and `[mid, hi]` is itself sorted, and for a sorted
half you can check in `O(1)` whether `target` could be inside it.

## Hint 2
Without duplicates, `nums[lo] <= nums[mid]` tells you the left half is
sorted. With duplicates there is a case where you cannot tell: when
`nums[lo] == nums[mid]` (think of `[3, 1, 3, 3, 3]` versus
`[3, 3, 3, 1, 3]`).

## Hint 3
In that ambiguous case `nums[lo]` is not the target (it equals `nums[mid]`,
which you already checked), so drop it with `lo += 1` and continue. Otherwise
use the sorted half: if `nums[lo] < nums[mid]` and
`nums[lo] <= target < nums[mid]`, search left; if `nums[lo] > nums[mid]` and
`nums[mid] < target <= nums[hi]`, search right; else search the other half.
