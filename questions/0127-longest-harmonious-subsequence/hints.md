# Hints

## Hint 1
Since a subsequence may skip elements and order does not affect max or min,
only how many times each value occurs matters.

## Hint 2
A harmonious subsequence uses exactly two values, `x` and `x + 1`, and the
longest one takes every copy of both.

## Hint 3
Count each value. For each value `x` such that `x + 1` is also present, the
candidate length is `count[x] + count[x + 1]`. Return the largest, or 0.
