# Approach: two lower-bound searches

In a sorted array the copies of `target` occupy the index range
`[lower_bound(target), lower_bound(target + 1))`, where `lower_bound(x)` is the
first index whose value is at least `x` (or `n` if there is none). Each lower
bound is one binary search over the half-open range `[lo, hi)`:
if `nums[mid] < x`, everything up to `mid` is too small, so `lo = mid + 1`;
otherwise `mid` could be the answer, so `hi = mid`.

If the first search lands past the end or on a different value, `target` is
absent. Otherwise the block ends just before the second bound.

```python
def search_range(nums, target):
    def first_at_least(x):
        lo, hi = 0, len(nums)
        while lo < hi:
            mid = (lo + hi) // 2
            if nums[mid] < x:
                lo = mid + 1
            else:
                hi = mid
        return lo

    first = first_at_least(target)
    if first == len(nums) or nums[first] != target:
        return [-1, -1]
    return [first, first_at_least(target + 1) - 1]
```

`target + 1` can be `10^9 + 1`, which still fits in a 32-bit integer.

## Complexity

- Time: O(log n), two binary searches.
- Space: O(1).

## Pitfalls

- Expanding outward from a single match. That is linear when `target`
  fills most of the array.
- Off-by-one errors in the bounds. A half-open `[lo, hi)` range with
  `hi = mid` / `lo = mid + 1` always terminates and returns a position in
  `[0, n]`.
- Reading `nums[first]` without checking `first < n`, including the empty
  array.
