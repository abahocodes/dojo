# Hints

## Hint 1
Multiplying `x` by itself `n` times is O(n), and `n` can be two billion. Notice
that `x^8 = ((x²)²)²` needs only three multiplications. How does that
generalise?

## Hint 2
`xⁿ = (x^(n/2))²` when `n` is even, and `x · (x^(n/2))²` when `n` is odd
(with integer division). Each step halves the exponent, so there are only
about 31 steps.

## Hint 3
Handle a negative exponent by switching to `1/x` and `-n` first. Careful:
`-(-2^31)` doesn't fit in a 32-bit `int`, so widen `n` to a 64-bit integer
before negating. Then loop: if the low bit of `n` is set, multiply the result
by `x`; square `x`; shift `n` right by one.
