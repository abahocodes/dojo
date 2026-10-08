# Approach: move everything to the median

If every element ends at value `t`, the cost is `sum(|nums[i] - t|)`. Shifting
`t` up by one adds 1 for each element `<= t` and subtracts 1 for each element
`> t`, so the cost decreases while fewer than half the elements are `<= t` and
increases afterwards. Any median minimizes it; for even `n`, every value
between the two middle elements ties, and the `n // 2`-th smallest is one of
them.

Sorting finds the median in O(n log n). Quickselect does it in O(n) on
average: partition around a random pivot into `< pivot`, `== pivot`,
`> pivot`, and keep only the part that contains index `n // 2`.

```python
import random

def min_moves_to_equal(nums):
    a = list(nums)
    k = len(a) // 2
    lo, hi = 0, len(a) - 1
    while lo < hi:
        pivot = a[random.randint(lo, hi)]
        lt, i, gt = lo, lo, hi
        while i <= gt:
            if a[i] < pivot:
                a[lt], a[i] = a[i], a[lt]
                lt += 1
                i += 1
            elif a[i] > pivot:
                a[i], a[gt] = a[gt], a[i]
                gt -= 1
            else:
                i += 1
        if k < lt:
            hi = lt - 1
        elif k > gt:
            lo = gt + 1
        else:
            break
    median = a[k]
    return sum(abs(v - median) for v in a)
```

## Complexity

- Time: O(n) on average (O(n^2) worst case, made vanishingly unlikely by the
  random pivot); O(n log n) if you simply sort.
- Space: O(n) for the working copy (O(1) extra if you may modify the input).

## Pitfalls

- Using the mean instead of the median. The mean minimizes squared
  distances, not absolute ones: for `[0, 0, 10]` the mean gives 13.33...
  (and 13 after rounding) while the median gives 10.
- Overflow: up to `10^5` elements, each up to `2 * 10^9` from the median.
  Accumulate in a 64-bit integer.
- A two-way partition with a fixed pivot is quadratic on many duplicates or
  sorted input; partition three ways and randomize the pivot.
