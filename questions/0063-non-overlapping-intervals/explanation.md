# Approach: greedy by earliest end

Maximise the number of intervals you keep; everything else is deleted. Among
the intervals compatible with what you have kept so far, always keep the one
that **ends** first. It blocks the least of the future, and any optimal
choice can be swapped for it without losing anything (the classic exchange
argument for interval scheduling).

```python
def erase_overlap_intervals(intervals):
    removed = 0
    last_end = float("-inf")
    for start, end in sorted(intervals, key=lambda iv: iv[1]):
        if start >= last_end:
            last_end = end      # keep it
        else:
            removed += 1        # it overlaps what we kept
    return removed
```

**Alternative:** sort by start and, when two intervals overlap, delete the one
that ends later (keep `last_end = min(last_end, end)`). Same complexity.

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted copy (O(1) extra if you sort in place).

## Pitfalls

- Sorting by start and keeping the first interval of each overlapping group
  fails: one long interval like `[1, 100]` can block many short ones.
- Use `>=`: intervals that only touch don't overlap.
- Return the number **removed**, not the number kept.
