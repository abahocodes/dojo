# Approach: binary search on the sorted half, shrinking on ties

Search the closed range `[lo, hi]`. Check `nums[mid]` first. If it is not the
target, decide which half is sorted:

- `nums[lo] < nums[mid]`: the left half `[lo, mid]` is sorted. If
  `nums[lo] <= target < nums[mid]` the target can only be there; otherwise it
  can only be in `[mid + 1, hi]`.
- `nums[lo] > nums[mid]`: the rotation point is inside `[lo, mid]`, so the
  right half `[mid, hi]` is sorted. If `nums[mid] < target <= nums[hi]` go
  right; otherwise go left.
- `nums[lo] == nums[mid]`: the left half might be all equal, or it might
  contain the rotation point; both `[3, 1, 3, 3, 3]` and `[3, 3, 3, 1, 3]`
  look the same at `lo` and `mid`. But `nums[lo]` equals `nums[mid]`, which is
  not the target, so `lo` can be discarded safely: `lo += 1`.

```python
def search_rotated_dups(nums, target):
    lo, hi = 0, len(nums) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if nums[mid] == target:
            return True
        if nums[lo] == nums[mid]:
            lo += 1
        elif nums[lo] < nums[mid]:
            if nums[lo] <= target < nums[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:
            if nums[mid] < target <= nums[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return False
```

## Complexity

- Time: O(log n) when ties are rare; O(n) in the worst case, e.g.
  `[1, 1, ..., 1, 2, 1, ..., 1]`, where only one step at a time can be
  discarded. No algorithm can do better there: the single different value
  could be anywhere.
- Space: O(1).

## Pitfalls

- Reusing the no-duplicates rule "`nums[lo] <= nums[mid]` means the left half
  is sorted". It is wrong for `[3, 1, 3, 3, 3]` with target `1`.
- Discarding a whole half when `nums[lo] == nums[mid]`. Only one element can
  be dropped safely.
- Using strict `<` on both ends of the range checks. The endpoints
  `nums[lo]` and `nums[hi]` themselves can be the target.
