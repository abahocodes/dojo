# Approach: sort by position, compare arrival times

A fleet always moves at the speed of its leading car, so its arrival time is
the leader's solo time `(target - position) / speed`. A car catches the fleet
directly ahead of it (no later than the destination) exactly when its solo
time is `<=` that fleet's arrival time. If it does, it merges and the fleet's
time stays the same. If it does not, it can never catch anything further
ahead either, so it leads a new fleet.

So walk the cars from closest-to-target to farthest, keeping the arrival
time of the last fleet formed. Each strictly slower arrival time starts a
new fleet. Equivalently, the arrival times of fleet leaders form a monotonic
stack when the cars are pushed in that order.

Times are fractions. Comparing `(target - p1) / s1 > (target - p2) / s2`
as `(target - p1) * s2 > (target - p2) * s1` keeps everything exact; the
products reach `10^12`, so use 64-bit integers.

```python
def car_fleet(target, position, speed):
    cars = sorted(zip(position, speed), reverse=True)
    fleets = 0
    lead_dist, lead_speed = 0, 1  # time 0: nothing ahead yet
    for p, s in cars:
        dist = target - p
        if dist * lead_speed > lead_dist * s:  # slower than the fleet ahead
            fleets += 1
            lead_dist, lead_speed = dist, s
    return fleets
```

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted pairs.

## Pitfalls

- Sorting by speed or by arrival time instead of position: only the car
  directly ahead on the road matters.
- Comparing with `<` instead of `<=` when merging: equal times mean the cars
  meet exactly at the destination, which counts as one fleet.
- Floating-point division can make two equal times compare unequal; the
  cross-multiplication is exact.
- `int` overflow in Java/C++: `10^6 * 10^6` needs `long`/`long long`.
