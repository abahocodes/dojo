# Approach: sort, then track the furthest end

Sort the intervals by start ascending, breaking ties by end **descending**.
After sorting, every interval that starts no later than the current one has
already been seen, and among intervals with the same start the longest comes
first.

Now scan and keep `max_end`, the largest end seen so far. The current interval
`[a, b)` is covered exactly when `b <= max_end`:

- If `b <= max_end`, the interval that reached `max_end` started at or before
  `a` (it came earlier) and ends at or after `b`, so it covers `[a, b)`. The
  two cannot be equal because all intervals are distinct.
- If `b > max_end`, no earlier interval reaches `b`, and no later interval
  starts at or before `a` (a later one with the same start is shorter thanks to
  the tie-break), so nothing covers it. It remains, and `max_end` becomes `b`.

```python
def remove_covered_intervals(intervals):
    ordered = sorted(intervals, key=lambda iv: (iv[0], -iv[1]))
    remaining = 0
    max_end = -1
    for _, end in ordered:
        if end > max_end:
            remaining += 1
            max_end = end
    return remaining
```

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted copy (O(log n) if you sort in place).

## Pitfalls

- Sorting ties by end ascending. Then `[1, 4)` is processed before `[1, 7)`,
  looks uncovered, and is wrongly counted.
- Comparing with `<` instead of `<=`: an interval with the same end as an
  earlier, longer one is still covered.
- Treating the half-open ends specially. Coverage only compares end points, so
  `[a, b)` behaves exactly like `[a, b]` here.
