# Hints

## Hint 1
The order of the picks doesn't matter, only how many come from each end. If
`i` cards come from the left, `k - i` come from the right.

## Hint 2
There are only `k + 1` possible splits. Can you move from one split to the
next in O(1)?

## Hint 3
Start with all `k` cards from the left. Then, `k` times, give back the
rightmost of the left cards and take one more card from the right end,
updating the total and keeping the maximum. (Equivalently: the cards left on
the table form a contiguous block of `n - k`; minimise its sum.)
