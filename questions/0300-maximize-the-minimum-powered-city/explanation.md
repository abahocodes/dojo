# Approach: binary search on the answer + greedy check

**Starting powers.** City `i` is supplied by the stations in cities
`[i - r, i + r]` (clipped to the row). A sliding window, or prefix sums, gives
all these sums in O(n).

**Monotonicity.** If a placement makes every city reach power `x`, the same
placement also reaches every `y < x`. So the answer is the largest feasible
`x`, and we can binary search for it. It lies between `min(power)` (build
nothing useful) and `min(power) + k` (each station adds at most 1 to the
weakest city).

**Feasibility of a target `x`.** Scan from left to right. When city `i` has
power `p < x`, it needs `x - p` more stations within range. Every city to its
left is already satisfied, so the best spot is as far right as possible while
still covering `i`: city `min(n - 1, i + r)`. Those stations cover cities up to
`i + 2r`. An exchange argument shows that any valid placement can be shifted
to this one without breaking anything, so if the greedy runs out of stations,
no placement works.

To apply the extra power in O(1), keep a running `extra` and a difference
array: when building `need` stations at step `i`, add `need` to `extra` now
and schedule `-need` at index `i + 2r + 1`.

```python
def max_min_power(stations, r, k):
    n = len(stations)
    power = [0] * n
    window = sum(stations[: min(n, r + 1)])
    for i in range(n):
        power[i] = window
        if i + r + 1 < n:
            window += stations[i + r + 1]
        if i - r >= 0:
            window -= stations[i - r]

    def feasible(target):
        added = [0] * (n + 1)
        extra = used = 0
        for i in range(n):
            extra += added[i]
            have = power[i] + extra
            if have < target:
                need = target - have
                used += need
                if used > k:
                    return False
                extra += need
                added[min(n, i + 2 * r + 1)] -= need
        return True

    lo, hi = min(power), min(power) + k
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if feasible(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
```

## Complexity

- Time: O(n log k). Each check is O(n), and the search range has size `k + 1`.
- Space: O(n) for the powers and the difference array.

## Pitfalls

- Overflow. Powers reach `10^5 * 10^5 = 10^10`, and the answer adds up to
  `10^9` more. Use 64-bit integers for powers, the running extra, the count of
  stations used and the binary-search bounds.
- Placing new stations at city `i` itself, or at `i - r`, wastes their reach
  to the left, where every city is already satisfied.
- Forgetting to clip the build location to `n - 1`. The difference-array index
  `i + 2r + 1` must also be clipped to `n`.
- Stop the check as soon as `used > k`. Otherwise `used` can grow very large
  for big targets.
- Use the upper midpoint `(lo + hi + 1) / 2` when the update is `lo = mid`, or
  the loop never ends.
