# Hints

## Hint 1
Only the number of copies of each value matters. Count them.

## Hint 2
If a value appears `c` times, its cards fill `c / X` piles, so `X` must
divide `c`. That must hold for every value at once.

## Hint 3
`X` must divide every count, so it divides their greatest common divisor.
A valid `X >= 2` exists exactly when that gcd is at least 2.
