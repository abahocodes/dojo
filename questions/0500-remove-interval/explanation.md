# Approach: cut each interval independently

Walk the intervals in order. For `[a, b)`:

- If `b <= lo` or `a >= hi`, it does not meet `[lo, hi)` (half-open intervals
  that only touch share no number). Keep it unchanged.
- Otherwise part of it is removed. The piece to the left of the cut,
  `[a, lo)`, survives if it is non-empty, i.e. `a < lo`. The piece to the
  right, `[hi, b)`, survives if `b > hi`.

Pieces are emitted left to right, and every piece lies inside its original
interval, so the output stays sorted and disjoint.

```python
def remove_interval(intervals, to_be_removed):
    cut_lo, cut_hi = to_be_removed
    result = []
    for a, b in intervals:
        if b <= cut_lo or a >= cut_hi:
            result.append([a, b])
            continue
        if a < cut_lo:
            result.append([a, cut_lo])
        if b > cut_hi:
            result.append([cut_hi, b])
    return result
```

## Complexity

- Time: O(n).
- Space: O(1) besides the output.

## Pitfalls

- Using `<` instead of `<=` in the untouched test: `[1, 3)` minus `[3, 6)` is
  still `[1, 3)`, with nothing removed.
- Emitting empty pieces such as `[lo, lo)` when the cut starts exactly at `a`.
- Forgetting the case where the cut sits strictly inside one interval and
  splits it into two.
