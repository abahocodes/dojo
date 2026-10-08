# Hints

## Hint 1
A subarray sum is the difference of two prefix sums. When is a difference of
two numbers divisible by `k`?

## Hint 2
When both numbers leave the same remainder modulo `k`. So the answer is the
number of pairs of prefix sums (including the empty prefix, `0`) that share a
remainder.

## Hint 3
Scan once, keeping a count of how many prefixes so far have each remainder.
Each new prefix pairs with all earlier ones of its remainder. Watch out:
in most languages `-1 % 4` is `-1`, not `3`; normalise it.
