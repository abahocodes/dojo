# Hints

## Hint 1
The order of `nums` does not matter, and duplicates cannot both be used in a
streak (`x * x > x` for `x >= 2`). Put the values in a set.

## Hint 2
From any start value `x`, the streak is forced: `x, x^2, x^4, ...` while the
next value is in the set. How long can such a chain be when values are at
most `10^5`?

## Hint 3
Since `2^32 > 10^5`, no chain is longer than 5, so following the chain from
every value is O(5n). Stop as soon as the next square exceeds the largest
value, and watch for overflow when squaring in 32-bit languages.
