# Approach: lower-bound binary search

The answer is the first index `i` with `nums[i] >= target` (or `len(nums)` if
there is none). If `target` is present that is its index; if not, it is
exactly where `target` would be inserted. So both cases are one search for
the boundary between "smaller than `target`" and "not smaller".

```python
def search_insert(nums, target):
    lo, hi = 0, len(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if nums[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    return lo
```

Invariant: everything before `lo` is `< target`, and everything from `hi` on
is `>= target`. The window `[lo, hi)` shrinks every step, and when it is
empty `lo` is the boundary. Starting with `hi = len(nums)` makes "insert at
the end" fall out naturally.

## Complexity

- Time: `O(log n)`.
- Space: `O(1)`.

## Pitfalls

- Starting with `hi = len(nums) - 1`: then a target larger than every
  element can never produce `len(nums)`.
- Mixing an inclusive window (`lo <= hi`) with `hi = mid`, which loops
  forever.
- Returning `-1` or `mid` when the value is absent; the question wants the
  insertion point.
