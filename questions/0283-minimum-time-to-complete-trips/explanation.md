# Approach: binary search on time

By time `t`, bus `i` has finished `t // time[i]` trips, so the fleet has
finished `trips(t) = sum(t // time[i])`. This is non-decreasing in `t`, so the
answer is the smallest `t` with `trips(t) >= total_trips`, which is a binary
search.

Bounds: `t = 1` is the smallest time worth trying, and
`min(time) * total_trips` always suffices because the fastest bus alone
completes `total_trips` trips by then.

```python
def minimum_time(time, total_trips):
    def enough(t):
        done = 0
        for x in time:
            done += t // x
            if done >= total_trips:
                return True
        return False

    lo, hi = 1, min(time) * total_trips
    while lo < hi:
        mid = (lo + hi) // 2
        if enough(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log(min(time) * total_trips)), about 47 iterations of an O(n)
  pass.
- Space: O(1).

## Pitfalls

- Overflow of the upper bound: `10^7 * 10^7 = 10^14` needs 64 bits.
- Overflow of the trip count: summing `t // time[i]` over `10^5` buses with
  `t` near `10^14` can exceed even 64 bits. Return early once the count reaches
  `total_trips`.
- Using `max(time) * total_trips` as the upper bound is correct but wastes
  iterations; using `total_trips` alone is wrong when every bus is slower
  than one unit.
