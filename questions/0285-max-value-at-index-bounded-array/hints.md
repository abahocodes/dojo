# Hints

## Hint 1
Suppose `nums[index] = v`. To keep the sum small, what should the other
elements look like?

## Hint 2
The cheapest array drops by 1 per step away from `index` until it reaches 1,
then stays at 1. Its sum is a closed-form function of `v`, the number of cells
to the left, and the number to the right.

## Hint 3
That minimum sum grows with `v`, so binary search the largest `v` whose
minimum sum is at most `max_sum`. For `L` cells on one side of a peak `v`: if
`L >= v - 1` they cost `(v - 1) * v / 2 + (L - v + 1)`, otherwise
`L * v - L * (L + 1) / 2`. Use 64-bit arithmetic.
