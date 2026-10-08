# Approach: binary search on the per-store cap

Fix a cap `x` on the units per store. Product type `i` then needs at least
`ceil(q_i / x)` stores, and that many are enough (fill stores with `x` units
until the remainder fits in one). Stores cannot be shared between types, so
the cap is achievable exactly when

    sum(ceil(q_i / x)) <= n.

The left-hand side is non-increasing in `x`, so the feasible caps form a
suffix of `1, 2, 3, ...`. We binary search for its first element. The cap
`max(quantities)` is always feasible because then every type needs one store
and `m <= n`.

```python
def minimized_maximum(n, quantities):
    def stores_needed(cap):
        return sum((q + cap - 1) // cap for q in quantities)

    lo, hi = 1, max(quantities)
    while lo < hi:
        mid = (lo + hi) // 2
        if stores_needed(mid) <= n:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(m log Q), where `Q = max(quantities)` (about 17 iterations).
- Space: O(1).

## Pitfalls

- Starting the search at `0`, which divides by zero. Every type has at least
  one unit, so the answer is at least `1`.
- Overflow: with `cap = 1` the total is up to `10^5 * 10^5 = 10^10`, so sum in
  64 bits (or stop early once the total exceeds `n`).
- Trying to balance units greedily store by store instead of fixing the cap
  first; the check becomes simple once `x` is fixed.
