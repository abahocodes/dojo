# Approach: binary search on the answer

The hours needed at speed `v` are `sum(ceil(p / v) for p in piles)`, which never
increases as `v` grows. So the speeds split into a "too slow" prefix and a "fast
enough" suffix, and we binary-search for the first fast-enough speed in
`[1, max(piles)]` (at `max(piles)` every pile takes exactly one hour, and
`h >= len(piles)`).

```python
def min_eating_speed(piles, h):
    lo, hi = 1, max(piles)
    while lo < hi:
        v = (lo + hi) // 2
        hours = sum((p + v - 1) // v for p in piles)
        if hours <= h:
            hi = v
        else:
            lo = v + 1
    return lo
```

## Complexity

- Time: O(n log M), where M is the largest pile.
- Space: O(1).

## Pitfalls

- Use the ceiling: a pile of 7 at speed 3 takes 3 hours, not 2.
- In languages with fixed-size integers, the hour total can exceed 32 bits
  (10^4 piles of 10^9 at speed 1); use a 64-bit or arbitrary-precision sum.
- Searching from `0` divides by zero; the lower bound is `1`.
