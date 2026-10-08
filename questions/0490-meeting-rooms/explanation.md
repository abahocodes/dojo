# Approach: sort by start, compare neighbours

Sort the meetings by start time. Now suppose meeting `a` overlaps some later
meeting `c`, i.e. `c.start < a.end`. Any meeting `b` sorted between them has
`b.start <= c.start < a.end`, so `a` overlaps `b` too. Repeating the argument,
any overlap implies an overlap between two meetings that are adjacent in
sorted order. So it is enough to check each meeting against the one before
it: they conflict exactly when `current.start < previous.end`.

```python
def can_attend_meetings(intervals):
    ordered = sorted(intervals)
    for i in range(1, len(ordered)):
        if ordered[i][0] < ordered[i - 1][1]:
            return False
    return True
```

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted copy (O(1) extra if you sort in place).

## Pitfalls

- Using `<=` in the comparison. A meeting ending at `t` and another starting
  at `t` do not conflict.
- Scanning without sorting. The input is in arbitrary order.
- Comparing only with the meeting immediately before in input order, or
  comparing ends with ends.
- Forgetting the empty calendar, which should return `true`.
