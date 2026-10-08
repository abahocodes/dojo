# Approach: binary search on the slope

Compare an element with its right neighbour. On the rising part of the
mountain `arr[i] < arr[i + 1]`; at the peak and everywhere after it
`arr[i] > arr[i + 1]`. So the predicate "`arr[i] > arr[i + 1]`" is false, false,
..., false, true, true, ... and the peak is the first index where it becomes
true. Binary search finds that boundary.

Keep the peak inside `[lo, hi]`. If `arr[mid] < arr[mid + 1]`, `mid` is on the
way up and the peak lies in `[mid + 1, hi]`. Otherwise `mid` is the peak or on
the way down and the peak lies in `[lo, mid]`. Each step shrinks the range and
the loop ends with `lo == hi` at the peak.

```python
def peak_index_in_mountain(arr):
    lo, hi = 0, len(arr) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if arr[mid] < arr[mid + 1]:
            lo = mid + 1
        else:
            hi = mid
    return lo
```

## Complexity

- Time: O(log n).
- Space: O(1).

## Pitfalls

- Setting `hi = mid - 1` in the "going down" branch. `mid` may be the peak
  itself, so it must stay in the range.
- Comparing with `arr[mid - 1]` while `mid` can be `0`, or with
  `arr[mid + 1]` when `mid` can be the last index. With `lo < hi` and a floor
  midpoint, `mid + 1 <= hi` is always valid.
- Returning the peak *value* instead of its index.
