# Hints

## Hint 1
Negative numbers rule out a sliding window: growing a window can make its sum
go down. Think in terms of prefix sums instead.

## Hint 2
A subarray ending at index `i` sums to `k` when some earlier prefix sum equals
`prefix(i) - k`. Which of those earlier prefixes gives the longest subarray?

## Hint 3
The earliest one. Keep a map from each prefix-sum value to the first index
where it occurred (the empty prefix `0` occurs at index `-1`). At each `i`,
look up `prefix - k`, then insert `prefix` only if it is not already there.
