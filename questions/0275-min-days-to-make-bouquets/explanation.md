# Approach: binary search on the day

**Impossible case.** The bouquets need `m * k` flowers in total. If
`m * k > n` it can never work. Otherwise, by the last bloom day every flower
is open and the first `m * k` flowers split into `m` adjacent groups of `k`,
so the answer exists and is at most `max(bloom_day)`. Compute `m * k` in
64-bit arithmetic: it can be as large as `10^11`.

**Counting for one day.** On day `d` a flower is usable when
`bloom_day[i] <= d`. Walk the row with a counter of consecutive usable
flowers; when it reaches `k`, form a bouquet and reset it to `0`; an unopened
flower also resets it. Greedily taking the leftmost `k` of each run is
optimal: a run of length `L` gives `L // k` bouquets either way.

**Monotonicity.** Flowers never close, so if day `d` works, every later day
works. Binary search the smallest working day in
`[min(bloom_day), max(bloom_day)]`. The answer is always one of the bloom
days.

```python
def min_days(bloom_day, m, k):
    if m * k > len(bloom_day):
        return -1

    def bouquets(day):
        made = run = 0
        for b in bloom_day:
            if b <= day:
                run += 1
                if run == k:
                    made += 1
                    run = 0
            else:
                run = 0
        return made

    lo, hi = min(bloom_day), max(bloom_day)
    while lo < hi:
        mid = (lo + hi) // 2
        if bouquets(mid) >= m:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log D), where `D = max(bloom_day) <= 10^9`: about 30 linear
  passes.
- Space: O(1).

## Pitfalls

- Overflow in `m * k` with 32-bit integers (`10^6 * 10^5`). Use a 64-bit
  product, or compare `m > n / k` instead.
- Counting open flowers instead of runs of adjacent open flowers.
- Forgetting to reset the run after forming a bouquet, which lets one
  flower count toward two bouquets.
- Computing `(lo + hi) / 2` in 32-bit when both are near `10^9`: the sum
  reaches `2 * 10^9`, past `2^31 - 1`. Use `lo + (hi - lo) / 2`.
