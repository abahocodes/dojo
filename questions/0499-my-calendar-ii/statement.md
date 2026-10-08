A calendar now tolerates double bookings but never triple bookings. Booking
requests arrive one at a time as half-open intervals `[start, end)`, meaning
the event occupies every moment `t` with `start <= t < end`.

Process `bookings` in order. A request is **accepted** unless adding it would
make some moment covered by three accepted bookings at once; otherwise it is
**rejected**. A rejected request is not stored. Events that merely touch, such
as `[10, 20)` and `[20, 30)`, share no moment.

Return a list with one boolean per request: `true` if it was accepted.

## Example 1

```
bookings = [[10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
output   = [true, true, true, false, true, true]
```

`[5, 15)` is rejected because `[10, 15)` is already booked twice. `[5, 10)`
ends exactly where `[10, 20)` begins, so it overlaps nothing. `[25, 55)` overlaps
`[10, 40)` and `[50, 60)` separately, never both at once.

## Example 2

```
bookings = [[1, 4], [2, 5], [3, 6], [4, 7]]
output   = [true, true, false, true]
```

## Constraints

- `1 <= len(bookings) <= 1000`
- `0 <= start < end <= 10^9`
