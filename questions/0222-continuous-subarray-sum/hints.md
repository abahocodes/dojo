# Hints

## Hint 1
Let `P(i)` be the sum of the first `i` elements. The subarray between two
prefix positions sums to `P(i) - P(j)`. When is that difference a multiple of
`k`?

## Hint 2
`P(i) - P(j)` is a multiple of `k` exactly when `P(i) mod k == P(j) mod k`.
So you are looking for two prefix positions with the same remainder.

## Hint 3
The positions must be at least two apart. Store the **first** position of every
remainder (remainder `0` at position `-1`, before the array) and report `true`
as soon as the current position is at least two past the stored one. Keep only
the remainder, so the running value never grows past `k`.
