# Hints

## Hint 1
Think about the very last move that lands you on step `n`. What step were you
standing on just before it?

## Hint 2
You arrived from step `n - 1` (with a one-step move) or from step `n - 2`
(with a two-step move). So `ways(n) = ways(n - 1) + ways(n - 2)`. Plain
recursion on that recomputes the same values exponentially many times.

## Hint 3
Build the answer bottom-up from `ways(0) = 1` and `ways(1) = 1`. You only ever
need the previous two values, so two variables are enough.
