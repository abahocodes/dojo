# Approach: sort the heaters, binary search per house

A radius `r` works exactly when every house is within `r` of its nearest
heater, so the answer is `max over houses of (distance to nearest heater)`.

After sorting `heaters`, the nearest heater to a house at `x` is one of two
neighbours: the first heater at a position `>= x` (found by binary search)
and the heater just before it. Take the smaller of those two distances
(ignoring a neighbour that does not exist) and track the maximum.

```python
from bisect import bisect_left

def find_radius(houses, heaters):
    heaters = sorted(heaters)
    best = 0
    for x in houses:
        i = bisect_left(heaters, x)
        near = float("inf")
        if i < len(heaters):
            near = heaters[i] - x
        if i > 0:
            near = min(near, x - heaters[i - 1])
        best = max(best, near)
    return best
```

Sorting both lists and sweeping with two pointers also works in the same
time bound.

## Complexity

- Time: O((n + m) log m) for `n` houses and `m` heaters.
- Space: O(m) for the sorted copy (O(1) extra if sorting in place).

## Pitfalls

- Only checking the heater on one side of the house.
- Index errors when the house lies before the first heater or after the last.
- Forgetting to sort: the input lists are in arbitrary order.
- Distances can reach `10^9 - 1`, which still fits in 32 bits, but
  `heaters[i] - x` must not be computed with a sentinel like `INT_MAX` in a
  way that overflows.
