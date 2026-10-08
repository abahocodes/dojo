# Hints

## Hint 1
Adding `seats` to every flight in the range costs O(n) per booking, which is
up to 4 * 10^8 steps in the worst case. Can a booking be recorded in O(1)?

## Hint 2
Think about how the per-flight total *changes* as you walk from flight to
flight. A booking only changes it in two places: where its range starts and
just after it ends.

## Hint 3
Keep a difference array `diff` of length `n + 1`: for each booking do
`diff[first - 1] += seats` and `diff[last] -= seats`. A running sum over
`diff[0..n-1]` then gives each flight's total.
