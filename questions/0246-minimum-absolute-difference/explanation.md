# Approach: sort, then scan adjacent gaps

In sorted order, any two non-adjacent elements have a strictly larger gap
than some adjacent pair between them, so only adjacent pairs can achieve the
minimum difference. Sort, find the smallest adjacent gap, and make a second
pass collecting every adjacent pair with that gap. The pairs are produced in
increasing order of their first element, which is the required order.

```python
def minimum_abs_difference(arr):
    a = sorted(arr)
    best = min(a[i + 1] - a[i] for i in range(len(a) - 1))
    return [[a[i], a[i + 1]] for i in range(len(a) - 1) if a[i + 1] - a[i] == best]
```

The two passes can be merged into one by resetting the result list whenever a
strictly smaller gap appears.

## Complexity

- Time: O(n log n) for the sort; both scans are O(n).
- Space: O(n) for the sorted copy and the output.

## Pitfalls

- Comparing all pairs: O(n^2) is too slow for `10^5` elements.
- Returning pairs in input order instead of ascending order.
- Gaps can reach `2 * 10^6`; that still fits a 32-bit int, but initialising
  the minimum with a small constant would be wrong.
