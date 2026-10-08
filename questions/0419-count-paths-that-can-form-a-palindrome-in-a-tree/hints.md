# Hints

## Hint 1
A multiset of letters can be rearranged into a palindrome exactly when at
most one letter appears an odd number of times. So only the **parity** of
each letter's count matters: a 26-bit mask.

## Hint 2
Let `mask[v]` be the parity mask of the letters on the path from the root to
`v`. The letters shared above the lowest common ancestor appear twice and
cancel, so the mask of the `u`-`v` path is `mask[u] ^ mask[v]`.

## Hint 3
Count pairs whose XOR has at most one set bit. Walk the nodes keeping a hash
map from mask to how many nodes had it. For node `v`, add the count of
`mask[v]` itself plus the counts of `mask[v] ^ (1 << b)` for all 26 letters,
then record `mask[v]`.
