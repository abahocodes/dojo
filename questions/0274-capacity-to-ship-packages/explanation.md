# Approach: binary search on the answer with a greedy check

**Checking one capacity.** Given a capacity `cap >= max(weights)`, simulate
the dock: add packages to the current day while they fit, and start a new day
when the next one does not. Filling each day as much as possible is optimal,
because leaving a package for later can only push all remaining packages
later. This gives the minimum number of days for `cap` in `O(n)`.

**Monotonicity.** A plan that works for `cap` also works for any larger
capacity, so "fits in `days` days" is false below some threshold and true
from it on. That threshold is the answer.

**Search range.** The capacity must be at least the heaviest package
(otherwise it can never be loaded), and `sum(weights)` always suffices
(everything in one day). Binary search between them.

```python
def ship_within_days(weights, days):
    def days_needed(cap):
        used, load = 1, 0
        for w in weights:
            if load + w > cap:
                used += 1
                load = 0
            load += w
        return used

    lo, hi = max(weights), sum(weights)
    while lo < hi:
        mid = (lo + hi) // 2
        if days_needed(mid) <= days:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log S), where `S = sum(weights) <= 2.5 * 10^7`, so about 25
  greedy passes.
- Space: O(1).

## Pitfalls

- Starting the search at `1` or `0`. A capacity below `max(weights)` makes
  the greedy loop count a day for a package that does not fit, giving a
  wrong (too small) day count instead of "impossible".
- Counting days wrong: start at one day, not zero, since the first package
  opens a day.
- Searching for "exactly `days` days". Fewer days is fine, which is what
  makes the predicate monotonic.
