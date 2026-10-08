# Hints

## Hint 1
For each item you want the first later price that is less than or equal to
it. Checking every later item works, but it is O(n^2) for `10^5` prices.

## Hint 2
Turn it around: when you reach price `p` at index `j`, which earlier items
does it discount? Exactly those that are still waiting and have price `>= p`.

## Hint 3
Keep a stack of indices still waiting for a discount. Their prices increase
from bottom to top. At each `j`, pop every index whose price is `>= prices[j]`
and subtract `prices[j]` from its result, then push `j`.
