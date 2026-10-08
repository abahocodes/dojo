# Hints

## Hint 1
A linear scan for the maximum works, but it ignores the shape. Pick any index
`i`: can you tell from `arr[i]` and one neighbour which side of the peak you
are on?

## Hint 2
If `arr[i] < arr[i + 1]` you are on the rising slope, so the peak is strictly
to the right of `i`. Otherwise you are at the peak or on the falling slope, so
the peak is at `i` or to its left.

## Hint 3
Binary search with `lo = 0, hi = n - 1`. While `lo < hi`, take `mid`; if
`arr[mid] < arr[mid + 1]` set `lo = mid + 1`, else `hi = mid`. When they meet,
`lo` is the peak. `mid + 1` is always in range because `mid < hi`.
