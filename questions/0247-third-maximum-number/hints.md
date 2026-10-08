# Hints

## Hint 1
Sorting and deduplicating works in O(n log n). Can you do it in a single
pass, keeping only a constant amount of state?

## Hint 2
Track the three largest distinct values seen so far: `first > second >
third`. When a new value arrives, where can it go, and what shifts down?

## Hint 3
Skip a value equal to any of the three. Mark empty slots with something no
input can equal (a `None`, or a 64-bit sentinel below `-2^31`): using
`INT_MIN` as "empty" breaks when `-2^31` is a real input.
