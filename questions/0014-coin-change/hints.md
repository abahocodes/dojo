# Hints

## Hint 1
Always grabbing the largest coin that fits is tempting, but it fails on
Example 1. Instead, think about the last coin in an optimal payout.

## Hint 2
If the last coin is `c`, the rest is an optimal payout of `amount - c`. So
`best(x) = 1 + min(best(x - c))` over every coin `c <= x`, with `best(0) = 0`.

## Hint 3
Fill an array `best[0..amount]` from left to right, using a sentinel such as
`amount + 1` to mean "unreachable". The answer is `best[amount]`, or `-1` if it
is still the sentinel.
