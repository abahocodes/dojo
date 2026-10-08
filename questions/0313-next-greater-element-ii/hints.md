# Hints

## Hint 1
Forget the circle first: for a plain array, the next greater element of every
index can be found in one pass with a stack of indices still waiting for an
answer.

## Hint 2
In that stack the waiting values never increase from bottom to top. When a new
value arrives, it answers every waiting index whose value is smaller.

## Hint 3
To handle the wrap-around, run the scan over `2n` positions, using index
`j % n`. Only push indices during the first `n` steps; the second lap exists
only to answer what is still waiting. Whatever never gets answered stays `-1`.
