# Hints

## Hint 1
Compare the middle element with the last element of the current range. If they
differ, which half must hold the minimum?

## Hint 2
If `nums[mid] < nums[hi]`, the stretch from `mid` to `hi` is sorted, so the
minimum is at `mid` or to its left. If `nums[mid] > nums[hi]`, the drop happens
after `mid`.

## Hint 3
When `nums[mid] == nums[hi]` you can't tell which side holds the drop (think of
`[3, 3, 1, 3]` versus `[3, 1, 3, 3]`). But `nums[hi]` has a copy at `mid`, so
discarding `hi` alone is safe: shrink `hi` by one and continue.
