# Approach: running sum with a maximum

Each altitude is the previous one plus the next gain, so the altitudes are
the prefix sums of `gain`, preceded by the starting `0`. Track the current
altitude and the maximum.

```python
def largest_altitude(gain):
    altitude = 0
    best = 0
    for g in gain:
        altitude += g
        best = max(best, altitude)
    return best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Starting `best` at the first altitude, or at negative infinity. The start
  (altitude `0`) is part of the route, so the answer is never below `0`.
- Returning the final altitude instead of the highest one.
