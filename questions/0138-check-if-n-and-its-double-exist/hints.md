# Hints

## Hint 1
Checking every pair works but is quadratic. Can a set of values seen so far
answer "does the partner exist?" in O(1)?

## Hint 2
When you reach `x`, its partner among earlier elements is either `2 * x`
(x is the half) or `x / 2` (x is the double, only possible when `x` is even).

## Hint 3
Check both before inserting `x` into the set. Inserting after checking is
what keeps a single `0` from matching itself, while a second `0` still finds
the first.
