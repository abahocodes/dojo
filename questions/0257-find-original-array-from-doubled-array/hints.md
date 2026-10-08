# Hints

## Hint 1
Look at the smallest value in `changed`. Can it be the double of anything
else in the array?

## Hint 2
It cannot (unless it is 0), so the smallest value must be in `original`, and
its double must be present to pair with it. Pair it, remove both, repeat.

## Hint 3
Values are at most `10^5`, so keep a count per value and walk values upward:
all `count[x]` copies of `x` need `count[x]` copies of `2x`. Zeros pair with
themselves, so their count must be even.
