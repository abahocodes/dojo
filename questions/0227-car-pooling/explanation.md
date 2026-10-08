# Approach: difference array

A trip adds `passengers` to the load on the half-open stretch `[start, end)`.
Rather than updating every kilometre in the stretch, record the change in load
at its two ends: `change[start] += passengers` and `change[end] -= passengers`.
The running sum of `change` from kilometre `0` up to `x` is then exactly the
number of people on board just after kilometre `x`. Because drop-offs and
pickups at the same kilometre are combined into one entry, a group leaving at
`x` frees its seats for a group boarding at `x`, as the statement requires.

```python
def car_pooling(trips, capacity):
    change = [0] * 1001
    for passengers, start, end in trips:
        change[start] += passengers
        change[end] -= passengers
    load = 0
    for delta in change:
        load += delta
        if load > capacity:
            return False
    return True
```

If the kilometre range were huge, the same idea works by sorting the `2t`
events by position (drop-offs before pickups at equal positions) and sweeping
them, in O(t log t).

## Complexity

- Time: O(t + L), where `t` is the number of trips and `L = 1001` the number
  of kilometre marks.
- Space: O(L).

## Pitfalls

- Treating `end` as still occupied. A group dropped off at `x` does not
  overlap with one picked up at `x`.
- In the sorted-events variant, processing pickups before drop-offs at the
  same position, which reports false overloads.
- Using `>=` instead of `>`: a car carrying exactly `capacity` people is
  fine.
