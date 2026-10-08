# Approach: ordered set of accepted bookings

Keep the accepted bookings sorted by start. Since they are pairwise disjoint,
their ends are sorted in the same order.

For a new request `[start, end)`, the accepted bookings that begin before `end`
form a prefix of that order. Bookings after the prefix begin at or after `end`,
so they cannot overlap. Inside the prefix, the last booking ends latest, so it
is the only one that can reach past `start`. The request is accepted exactly
when the prefix is empty or its last booking ends at or before `start`.

A binary search finds the prefix length. In Java and C++, an ordered map
(`TreeMap.lowerEntry(end)`, `map::lower_bound(end)` then one step back) does
the same.

```python
from bisect import bisect_left

def book_calendar(bookings):
    starts, ends, result = [], [], []
    for start, end in bookings:
        i = bisect_left(starts, end)
        if i > 0 and ends[i - 1] > start:
            result.append(False)
            continue
        starts.insert(i, start)
        ends.insert(i, end)
        result.append(True)
    return result
```

## Complexity

- Time: O(n log n) with a balanced ordered map. With sorted arrays the search
  is O(log n) but the insertion shifts elements, O(n) in the worst case;
  for `n <= 1000` either is instant. Checking every accepted booking is
  O(n^2), which also passes these limits.
- Space: O(n).

## Pitfalls

- Treating touching events as overlapping. `[10, 20)` and `[20, 30)` are fine.
- Storing rejected requests. They must not block later ones.
- Looking only at the booking just *after* the new start, or only the one just
  before it. The search above checks the one neighbour that matters.
