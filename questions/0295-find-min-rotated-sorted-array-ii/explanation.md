# Approach: binary search, shrinking on ties

Keep a range `[lo, hi]` that always contains a position of the minimum, and
compare `nums[mid]` with `nums[hi]`:

- `nums[mid] < nums[hi]`: `mid..hi` is sorted, so the minimum is at `mid` or
  earlier. Set `hi = mid`.
- `nums[mid] > nums[hi]`: the rotation's drop lies after `mid`, so the minimum
  is in `mid + 1..hi`. Set `lo = mid + 1`.
- Equal: the drop could be on either side. Since `nums[mid]` holds the same
  value as `nums[hi]`, dropping index `hi` can't lose the minimum value. Set
  `hi -= 1`.

```python
def find_min_dups(nums):
    lo, hi = 0, len(nums) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if nums[mid] < nums[hi]:
            hi = mid
        elif nums[mid] > nums[hi]:
            lo = mid + 1
        else:
            hi -= 1
    return nums[lo]
```

## Complexity

- Time: O(log n) when values are mostly distinct; O(n) in the worst case, such
  as `[1, 1, 1, ..., 0, 1]`, where every probe is a tie. No algorithm can do
  better there: the single smaller value could be anywhere.
- Space: O(1).

## Pitfalls

- Comparing against `nums[lo]` instead of `nums[hi]` breaks on arrays that
  aren't rotated at all.
- On a tie, discarding a whole half is wrong: `[3, 3, 1, 3]` and `[3, 1, 3, 3]`
  look the same at `lo`, `mid` and `hi`.
- `hi = mid - 1` in the first branch can skip the minimum when it sits at
  `mid`.
