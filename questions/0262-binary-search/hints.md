# Hints

## Hint 1
Look at the middle element. If it is smaller than `target`, which half of the
array can you throw away?

## Hint 2
Keep a window `[lo, hi]` of indices that could still hold `target`. Compare
`nums[mid]` with `target` and shrink the window to one side of `mid`.

## Hint 3
Loop while `lo <= hi`: return `mid` on a match, set `lo = mid + 1` if
`nums[mid] < target`, otherwise `hi = mid - 1`. When the window is empty the
target is absent, so return `-1`.
