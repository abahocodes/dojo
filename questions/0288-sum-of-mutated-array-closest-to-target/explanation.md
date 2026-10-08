# Approach: binary search where the capped sum crosses the target

Let `f(v) = sum(min(a, v) for a in arr)`. Raising the cap never lowers any
element, so `f` is non-decreasing; above `max(arr)` it stops changing. A
negative cap is never useful: `f(0) = 0` already beats every negative sum.

Because `f` is monotone, `|f(v) - target|` decreases until `f` reaches
`target` and increases afterwards. Binary search the smallest `v` in
`[0, max(arr)]` with `f(v) >= target`:

- if there is none, every cap undershoots and the largest useful one,
  `max(arr)`, is best (larger caps tie but are not smaller);
- otherwise the best cap is `v` or `v - 1`, whichever is closer, with `v - 1`
  winning ties because it is smaller.

```python
def find_best_value(arr, target):
    def capped(v):
        return sum(min(a, v) for a in arr)

    lo, hi = 0, max(arr)
    if capped(hi) < target:
        return hi
    while lo < hi:
        mid = (lo + hi) // 2
        if capped(mid) >= target:
            hi = mid
        else:
            lo = mid + 1
    if lo > 0 and target - capped(lo - 1) <= capped(lo) - target:
        return lo - 1
    return lo
```

Sorting `arr` with prefix sums makes each `f(v)` evaluation O(log n), but the
plain O(n) evaluation is already fast enough here.

## Complexity

- Time: O(n log M), where M = max(arr) <= 10^5 (about 17 passes).
- Space: O(1).

## Pitfalls

- Searching only caps that appear in `arr`; the best cap is usually between
  two elements.
- Breaking the tie towards the larger cap. Both the `v - 1` vs `v` comparison
  and the "no cap reaches target" case must return the smaller value.
- Forgetting that `v = 0` can win: `arr = [1, 1, 1, 1]`, `target = 1` gives
  sums 0 (cap 0) and 4 (cap 1), so the answer is `0`.
