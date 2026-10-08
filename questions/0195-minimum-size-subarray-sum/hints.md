# Hints

## Hint 1
Checking every subarray is O(n^2). Use the fact that all numbers are positive:
extending a subarray always increases its sum, shrinking it always decreases
the sum.

## Hint 2
Use two pointers, `left` and `right`, bounding a window. Grow the window by
moving `right`. Once its sum reaches `target`, there is no point in growing it
further from the same `left`.

## Hint 3
After adding `nums[right]`, while the window sum is at least `target`, record
`right - left + 1` and drop `nums[left]` (moving `left` forward). Each index
enters and leaves the window at most once, so the whole scan is O(n).
