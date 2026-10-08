# Hints

## Hint 1
Let `c` be the number of `1`s. In the final arrangement they fill some
circular window of exactly `c` consecutive positions.

## Hint 2
For a fixed window, every `0` inside it must be swapped with a `1` outside it,
and one swap fixes one such pair. So the cost of a window is the number of
`0`s it contains.

## Hint 3
Slide a window of length `c` around the circle (index with `i % n`, or walk the
array twice) keeping a running count of `1`s inside. The answer is
`c - max_ones_in_window`.
