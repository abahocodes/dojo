# Hints

## Hint 1
A list of `n` distinct values has `n!` arrangements. How many choices do you
have for the first position? And for the second, once the first is fixed?

## Hint 2
Build each permutation one position at a time. At every step, try each value
that hasn't been placed yet, then undo the choice and try the next one.

## Hint 3
Backtracking: keep `current` and a `used` flag per index. When `current` has
`n` values, record a **copy**. Otherwise, for each unused index mark it,
append the value, recurse, then pop and unmark.
