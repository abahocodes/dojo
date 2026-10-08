# Approach: binary search on the answer with a greedy check

Sort the positions. For a candidate gap `g`, the greedy check places the first
ball in the leftmost basket and each next ball in the first basket at least
`g` past the previous ball. Greedy is optimal: any valid placement can be
shifted, ball by ball, onto the greedy baskets without breaking the gap, so if
any placement fits `m` balls the greedy one does too.

Feasibility is monotone (a placement valid for `g` is valid for every smaller
gap), so we binary search the largest feasible `g`. The answer is at least `1`
(positions are distinct) and at most `(max - min) // (m - 1)`, since `m - 1`
gaps must fit inside the total span.

```python
def max_min_distance(position, m):
    pos = sorted(position)

    def fits(gap):
        placed, last = 1, pos[0]
        for p in pos[1:]:
            if p - last >= gap:
                placed += 1
                last = p
                if placed == m:
                    return True
        return False

    lo, hi = 1, (pos[-1] - pos[0]) // (m - 1)
    while lo < hi:
        mid = (lo + hi + 1) // 2   # round up so lo = mid always makes progress
        if fits(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
```

## Complexity

- Time: O(n log n + n log D), where `D = max - min <= 10^9` (about 30
  iterations of the O(n) check).
- Space: O(n) for the sorted copy (O(1) extra if sorting in place).

## Pitfalls

- Forgetting to sort: the input order is arbitrary.
- Rounding the midpoint down while updating with `lo = mid`, which loops
  forever when `hi = lo + 1`.
- Overflow in `lo + hi` if the upper bound were near `2^31`; with
  `hi <= 10^9` it is safe, but `lo + (hi - lo + 1) / 2` avoids the question.
