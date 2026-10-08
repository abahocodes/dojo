# Hints

## Hint 1
Sorting gives an O(n log n) answer. For linear time, think about what the
largest gap must be at least, given the minimum, the maximum and `n`.

## Hint 2
With `n` values spread over `[lo, hi]`, the `n - 1` gaps add up to `hi - lo`,
so the largest gap is at least `(hi - lo) / (n - 1)`.

## Hint 3
Split `[lo, hi]` into buckets narrower than that bound. Two values in the same
bucket can never form the largest gap, so keep only each bucket's minimum and
maximum, then compare each non-empty bucket's minimum with the previous
non-empty bucket's maximum.
