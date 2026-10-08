# Approach: each attack contributes until the next one

Charge every poisoned second to the attack that was most recently applied.
Attack `i` owns the seconds from `time_series[i]` until either its poison
expires or attack `i + 1` lands, whichever comes first:

- if `time_series[i + 1] - time_series[i] >= duration`, it owns all
  `duration` seconds;
- otherwise attack `i + 1` restarts the timer, and attack `i` owns only the
  `time_series[i + 1] - time_series[i]` seconds before it.

The last attack is never interrupted, so it owns `duration` seconds. Every
poisoned second has exactly one owner, so the sum is the answer.

```python
def find_poisoned_duration(time_series, duration):
    total = 0
    for i in range(len(time_series) - 1):
        total += min(duration, time_series[i + 1] - time_series[i])
    return total + duration
```

Repeated attack times give a gap of 0, which correctly adds nothing.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Adding `duration` for every attack and double-counting overlaps.
- Simulating second by second: times and durations go up to `10^7`, so a
  per-second set or array is far too slow and large.
- Forgetting the last attack's full `duration`.
