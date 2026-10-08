# Hints

## Hint 1
Write `f(v)` for the capped sum. How does `f(v)` change as `v` grows? Is there
any point in trying a cap larger than `max(arr)`?

## Hint 2
`f` is non-decreasing, and constant from `max(arr)` on. So the best cap sits
right where `f` crosses `target`.

## Hint 3
Binary search the smallest `v` in `[0, max(arr)]` with `f(v) >= target`.
The answer is `v` or `v - 1`: compare `|f(v) - target|` with
`|f(v - 1) - target|` and prefer `v - 1` on a tie. If no cap reaches the
target, return `max(arr)`.
