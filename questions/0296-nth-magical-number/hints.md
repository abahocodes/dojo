# Hints

## Hint 1
Walking through the multiples one by one takes up to 10^9 steps. Instead, how
many magical numbers are at most some value `x`?

## Hint 2
Inclusion-exclusion: `x // a + x // b - x // lcm(a, b)`. The last term removes
numbers counted twice because they are multiples of both.

## Hint 3
That count is non-decreasing in `x`, and the answer lies in
`[min(a, b), n * min(a, b)]`. Binary search for the smallest `x` whose count
reaches `n`, using 64-bit arithmetic, and only reduce modulo `10^9 + 7` at the
very end.
