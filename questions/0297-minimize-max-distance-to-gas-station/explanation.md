# Approach: binary search on the maximum distance

Splitting a gap of length `g` into pieces of length at most `D` needs
`ceil(g / D) - 1` new stations, and spacing them evenly achieves it. So `D` is
achievable exactly when the sum of those counts over all gaps is at most `k`.
Using exactly `k` stations is never a problem: extra stations can go anywhere
without lengthening a piece.

The needed count shrinks as `D` grows, so binary search on `D`. Over the reals,
replacing `ceil(g / D) - 1` with `floor(g / D)` changes only the boundary
points, not where the search converges, and it is simpler to compute.

```python
def minmax_gas_dist(stations, k):
    gaps = [b - a for a, b in zip(stations, stations[1:])]

    def fits(limit):
        added = 0
        for g in gaps:
            added += int(g / limit)
            if added > k:
                return False
        return True

    lo, hi = 0.0, float(max(gaps))
    for _ in range(100):
        mid = (lo + hi) / 2
        if fits(mid):
            hi = mid
        else:
            lo = mid
    return hi
```

## Complexity

- Time: O(n * I), with I = 100 iterations of the search.
- Space: O(n) for the gaps (O(1) if you read them from `stations` directly).

## Pitfalls

- Greedily handing each new station to the gap with the longest current piece
  is correct but costs O(k log n) with `k` up to 10^6. It must also re-divide
  that whole gap evenly; halving the longest piece is wrong.
- Small candidates make `g / D` huge. Stop summing once the count exceeds `k`,
  and use a 64-bit counter.
- Looping `while hi - lo > 1e-6` can spin forever on large values where adjacent
  doubles are further apart than the epsilon; a fixed iteration count avoids it.
