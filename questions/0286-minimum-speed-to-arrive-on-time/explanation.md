# Approach: binary search on the speed, in exact integer arithmetic

At speed `s`, train `i < n - 1` effectively costs `ceil(dist[i] / s)` whole
hours (the ride plus the wait for the next departure), and the last train
costs `dist[n-1] / s` hours exactly. The total never increases as `s` grows,
so the smallest good speed can be found by binary search on `[1, 10^7]`.

Floating-point time sums can be off by a hair at an exact boundary (for
example `hour = 2.01`). Since `hour` has two decimals, scale everything by
100: `H = round(hour * 100)` is exact. With `S = sum(ceil(dist[i] / s))` over
the first `n - 1` trains, speed `s` is on time when

- `100 * S <= H`, and
- `dist[n-1] / s <= (H - 100 * S) / 100`, i.e.
  `100 * dist[n-1] <= R * s` with `R = H - 100 * S`.

If `R >= 100 * dist[n-1]` the second test passes for any `s >= 1`, so the
product `R * s` is only formed when `R < 10^7`, which keeps it small.

```python
def min_speed_on_time(dist, hour):
    total = round(hour * 100)
    last = dist[-1] * 100

    def on_time(speed):
        whole = 0
        for d in dist[:-1]:
            whole += (d + speed - 1) // speed
        rest = total - whole * 100
        return rest >= 0 and (rest >= last or last <= rest * speed)

    lo, hi = 1, 10**7
    if not on_time(hi):
        return -1
    while lo < hi:
        mid = (lo + hi) // 2
        if on_time(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log 10^7), about 24 passes over `dist`.
- Space: O(1).

## Pitfalls

- Rounding up the last train too. Nothing departs after it, so its time
  stays fractional.
- Comparing floating-point sums to `hour`: `1/3 + ...` style sums carry
  rounding error. Scaling by 100 makes the comparison exact.
- Writing `hour * 100` without rounding: `2.01 * 100` is `200.99999...`
  in binary floating point.
- When `hour <= n - 1` no speed can work, because each of the first `n - 1`
  trains costs at least one whole hour.
