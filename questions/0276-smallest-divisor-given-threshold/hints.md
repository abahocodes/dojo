# Hints

## Hint 1
As `d` grows, each rounded quotient `ceil(x / d)` can only stay the same or
shrink. What does that say about the sum as a function of `d`?

## Hint 2
The sum is non-increasing in `d`, so the divisors that work form a suffix of
`1, 2, 3, ...`. You want the first divisor of that suffix: binary search for it.

## Hint 3
Search `d` in `[1, max(nums)]` (at `d = max(nums)` the sum is `len(nums)`,
which always fits). For each candidate compute `sum((x + d - 1) // d)`; if it
is at most `threshold`, move the upper bound down to `d`, otherwise move the
lower bound to `d + 1`.
