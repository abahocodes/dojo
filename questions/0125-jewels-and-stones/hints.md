# Hints

## Hint 1
For every stone you need to answer one question: is this character one of
the jewel kinds?

## Hint 2
Scanning `jewels` for every stone works, but a structure with constant-time
membership makes each check O(1).

## Hint 3
Put the characters of `jewels` into a set (or a boolean array indexed by
character code), then count the stones whose character is in it.
