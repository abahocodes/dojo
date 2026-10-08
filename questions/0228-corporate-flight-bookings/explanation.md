# Approach: difference array

Instead of touching every flight a booking covers, record only where the
running total goes up and where it goes back down. With 0-based positions,
booking `[first, last, seats]` raises the total at index `first - 1` and
lowers it again at index `last`. After all bookings are recorded, a prefix sum
turns those changes back into totals.

```python
def corp_flight_bookings(bookings, n):
    diff = [0] * (n + 1)
    for first, last, seats in bookings:
        diff[first - 1] += seats
        diff[last] -= seats
    totals = []
    running = 0
    for i in range(n):
        running += diff[i]
        totals.append(running)
    return totals
```

The extra slot `diff[n]` absorbs the decrement for bookings that end on the
last flight, so no bounds check is needed.

## Complexity

- Time: O(n + b) for `b` bookings.
- Space: O(n) for the difference array (plus the output).

## Pitfalls

- Off-by-one with 1-based labels: the increment goes at `first - 1`, the
  decrement at `last` (the slot right after the range).
- Sizing the array as `n` instead of `n + 1` and indexing past the end when
  `last == n`.
- Overflow is not a concern: the largest total is 2 * 10^4 * 10^4 = 2 * 10^8,
  which fits in a 32-bit int.
