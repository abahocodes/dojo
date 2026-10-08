# Hints

## Hint 1
Flip the question around: given a cap `C`, can you decide whether some cut into
at most `k` pieces keeps every piece sum at or below `C`?

## Hint 2
For a fixed cap, a greedy scan works: keep extending the current piece and
start a new one only when adding the next element would exceed the cap. That
uses the fewest pieces possible. A cut with fewer than `k` pieces can always be
split further without raising the load.

## Hint 3
"Feasible with cap `C`" is monotone in `C`. Binary search `C` between
`max(nums)` (no piece can be smaller than its largest element) and `sum(nums)`
(one piece holds everything), and return the smallest feasible cap.
