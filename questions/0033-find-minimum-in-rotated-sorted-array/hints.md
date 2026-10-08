# Hints

## Hint 1
`min(nums)` is O(n). The list is made of two increasing runs, and the answer is
the first element of the second run. Can you tell which run a position is in by
comparing it to a single other element?

## Hint 2
Compare `nums[mid]` with the last element `nums[hi]`. If `nums[mid] > nums[hi]`,
the drop happens somewhere to the right of `mid`.

## Hint 3
Binary search with `lo, hi = 0, n - 1` while `lo < hi`: if
`nums[mid] > nums[hi]` set `lo = mid + 1`, otherwise set `hi = mid` (mid itself
might be the minimum). When the loop ends, `nums[lo]` is the answer.
