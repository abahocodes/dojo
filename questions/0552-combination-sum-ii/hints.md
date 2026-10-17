# Hints

## Hint 1
This is Combination Sum with two changes: each card is used at most once,
and equal cards make the same combination. Start from the start-index
backtracking used there.

## Hint 2
Using each card once means recursing on `i + 1` instead of `i`. On its own
that still lists `[1, 5]` twice when there are two `1` cards. Sort first so
equal values sit together.

## Hint 3
In the loop over `i = start, start + 1, ...`, skip `candidates[i]` when
`i > start` and it equals `candidates[i - 1]`: that value was already tried
as the next pick at this depth. Because the list is sorted, you can also stop
the loop as soon as a candidate exceeds the remaining sum.
