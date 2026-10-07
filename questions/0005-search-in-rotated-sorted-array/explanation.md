# Approach: binary search on the sorted half

Any midpoint splits the range into two pieces, and at least one of them contains
no "drop" (it is plainly sorted). For the sorted piece we can test in O(1)
whether the target lies within its bounds; if it does, keep that piece,
otherwise keep the other one.

```python
def search(nums, target):
    lo, hi = 0, len(nums) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if nums[mid] == target:
            return mid
        if nums[lo] <= nums[mid]:            # left half sorted
            if nums[lo] <= target < nums[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:                                # right half sorted
            if nums[mid] < target <= nums[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return -1
```

## Complexity

- Time: O(log n) — the range halves every step.
- Space: O(1).

## Pitfalls

- Use `<=` in `nums[lo] <= nums[mid]`: when `lo == mid` the one-element left
  half counts as sorted.
- Getting the strict/non-strict bounds wrong in the range checks leads to
  infinite loops or missed targets at the edges.
- Alternative: first binary-search for the rotation point (the minimum), then
  run an ordinary binary search on the correct side. Same complexity, two passes.
