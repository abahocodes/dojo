# Hints

## Hint 1
Listing and sorting all fractions costs O(n² log n). Instead, guess a value
`x` and ask: how many fractions are smaller than `x`?

## Hint 2
For a fixed denominator `arr[j]`, the numerators below `x * arr[j]` form a
prefix of `arr`. As `j` increases, that prefix only grows, so a single pointer
counts every fraction below `x` in O(n). Track the largest fraction found
below `x` at the same time.

## Hint 3
Binary search `x` over `(0, 1)`. If exactly `k` fractions are below `x`, the
largest of them is the answer. If fewer, raise `lo`; if more, lower `hi`.
Distinct fractions differ by at least `1 / (3*10^4)^2`, so doubles are precise
enough and the search finishes after a few dozen rounds.
