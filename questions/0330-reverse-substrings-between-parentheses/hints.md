# Hints

## Hint 1
A direct approach: keep a stack of partial strings. On `"("` start a new one; on
`")"` reverse the current one and append it to the one below. This is O(n^2)
in the worst case, which is fine for `n = 2000`, but there is a linear method.

## Hint 2
Reversing a segment twice restores it. Instead of physically reversing, imagine
reading the string with a cursor that can change direction.

## Hint 3
Pre-compute the matching partner of every parenthesis with a stack. Then walk
from index 0 moving right; whenever you land on a parenthesis, jump to its
partner and flip the direction of travel. Output every letter you pass. Stop
when the cursor leaves the string.
