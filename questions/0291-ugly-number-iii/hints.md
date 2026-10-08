# Hints

## Hint 1
Generating ugly numbers one by one is too slow when `n` is near 10^9. Can
you count how many ugly numbers are `<= x` without listing them?

## Hint 2
Inclusion-exclusion: the count of numbers `<= x` divisible by `a`, `b` or `c`
is `x/a + x/b + x/c - x/lcm(a,b) - x/lcm(a,c) - x/lcm(b,c) + x/lcm(a,b,c)`
(integer division).

## Hint 3
That count is non-decreasing in `x`, so binary search the smallest `x` in
`[1, 2 * 10^9]` whose count is at least `n`. Compute each lcm as
`p / gcd(p, q) * q` in 64-bit arithmetic, capping it once it exceeds
`2 * 10^9` (anything larger contributes 0 to the count anyway).
