# Approach: sort by start, then sweep

Once intervals are sorted by start, each one either overlaps the interval we
are currently building (its start is `<=` the current end) or begins a new,
disjoint group. Because later intervals start even further right, a closed
group can never be touched again.

```python
def merge(intervals):
    intervals.sort(key=lambda iv: iv[0])
    merged = []
    for s, e in intervals:
        if merged and s <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], e)
        else:
            merged.append([s, e])
    return merged
```

## Complexity

- Time: O(n log n) for the sort; the sweep is O(n).
- Space: O(n) for the output (plus the sort's own space).

## Pitfalls

- Using `<` instead of `<=`: touching intervals like `[1, 3]` and `[3, 6]` must merge.
- Setting the end to `e` rather than `max(end, e)`: a contained interval such as
  `[2, 3]` inside `[1, 10]` would shrink the group.
- Forgetting to sort first — the input order is arbitrary.
