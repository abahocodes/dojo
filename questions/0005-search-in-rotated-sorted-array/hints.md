# Hints

## Hint 1
A linear scan is O(n). To get O(log n) you want binary search — but the list
isn't globally sorted. What can you still say about it?

## Hint 2
Cut the range at any midpoint: at least one of the two halves is sorted in the
normal way. You can tell which by comparing the endpoints with the middle.

## Hint 3
With `lo`, `mid`, `hi`: if `nums[lo] <= nums[mid]`, the left half is sorted —
search it if `nums[lo] <= target < nums[mid]`, else go right. Otherwise the
right half is sorted — search it if `nums[mid] < target <= nums[hi]`, else go left.
