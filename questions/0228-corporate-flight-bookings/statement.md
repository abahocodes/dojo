An airline runs `n` flights labeled `1` through `n`. You receive a list of
group reservations, where `bookings[i] = [first, last, seats]` means that the
group reserved `seats` seats on **every** flight from `first` to `last`,
inclusive.

Return an array of length `n` whose element at index `i` is the total number
of seats reserved on flight `i + 1` (so the answer is listed in label order).

## Example 1

```
bookings = [[1, 3, 5], [2, 4, 7]]
n        = 4
output   = [5, 12, 12, 7]   # flights 2 and 3 are covered by both groups
```

## Example 2

```
bookings = [[2, 2, 9]]
n        = 3
output   = [0, 9, 0]
```

## Constraints

- `1 <= n <= 2 * 10^4`
- `1 <= len(bookings) <= 2 * 10^4`
- `bookings[i].length == 3`
- `1 <= first <= last <= n`
- `1 <= seats <= 10^4`
