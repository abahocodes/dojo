# Hints

## Hint 1
The order of the operations does not matter: each value `x` can only ever be
paired with the value `k - x`. Can you pair greedily?

## Hint 2
Scan the array once and remember the values that are still waiting for a
partner. When `k - x` is waiting, pairing it with `x` right away is never
worse than waiting.

## Hint 3
Keep a hash map from value to how many unpaired copies you have seen. For
each `x`: if the count of `k - x` is positive, decrement it and count an
operation; otherwise increment the count of `x`. (Sorting and closing in
with two pointers also works in `O(n log n)`.)
