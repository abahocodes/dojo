You are running a calendar that never allows two events to overlap. Booking
requests arrive one at a time as half-open intervals `[start, end)`, meaning
the event occupies every moment `t` with `start <= t < end`.

Process `bookings` in order. A request is **accepted** if it shares no moment
with any previously accepted booking; events that merely touch, such as
`[10, 20)` and `[20, 30)`, do not overlap. Otherwise it is **rejected**, and a
rejected request is not stored, so it never blocks later requests.

Return a list with one boolean per request: `true` if it was accepted.

## Example 1

```
bookings = [[10, 20], [15, 25], [20, 30]]
output   = [true, false, true]
```

## Example 2

```
bookings = [[5, 8], [1, 5], [8, 9], [0, 10], [6, 7]]
output   = [true, true, true, false, false]
```

## Constraints

- `1 <= len(bookings) <= 1000`
- `0 <= start < end <= 10^9`
