# Hints

## Hint 1
When is it impossible? If one letter appears more than `ceil(n / 2)` times,
two copies must end up next to each other. Otherwise some arrangement always
exists (for instance, place the most frequent letter at even positions first).

## Hint 2
To get the lexicographically smallest arrangement, build it one position at a
time and always try the smallest letter first. The only question is whether a
choice leaves the remaining letters arrangeable.

## Hint 3
After placing letter `c` with `r` letters left, the rest is still arrangeable
(and can start with something other than `c`) exactly when every remaining
count is at most `ceil(r / 2)` and `c`'s remaining count is at most
`floor(r / 2)`. Try letters `a` to `z` (skipping the previous one) and take the
first that passes this test.
