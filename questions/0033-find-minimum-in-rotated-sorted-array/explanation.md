# Approach: binary search against the right end

The rotated list consists of a high run followed by a low run, and every value
in the high run is larger than every value in the low run — in particular larger
than the last element. So comparing a position with the right end of the search
range tells us which run it is in.

- `nums[mid] > nums[hi]`: `mid` is in the high run, the minimum is strictly to
  the right: `lo = mid + 1`.
- otherwise `mid` is in the low run (or the range is already sorted), so the
  minimum is at `mid` or to its left: `hi = mid`.

```python
def find_min(nums):
    lo, hi = 0, len(nums) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if nums[mid] > nums[hi]:
            lo = mid + 1
        else:
            hi = mid
    return nums[lo]
```

## Complexity

- Time: O(log n) — the range halves every step.
- Space: O(1).

## Pitfalls

- Comparing with `nums[lo]` instead of `nums[hi]` breaks on an unrotated list,
  where both halves look "sorted".
- Use `hi = mid`, not `hi = mid - 1`: `mid` may be the minimum itself.
- The loop condition is `lo < hi`; with `<=` and `hi = mid` it never terminates.
