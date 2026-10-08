# Hints

## Hint 1
Searching the tree directly costs O(height) per query, and a lopsided tree can
be 10^5 deep. Can you turn the tree into a structure with guaranteed fast
lookups?

## Hint 2
An in-order traversal gives the values as a sorted array. Floor and ceiling in
a sorted array are a binary search away.

## Hint 3
Find `i`, the first index with `values[i] >= q` (lower bound). If
`values[i] == q`, answer `[q, q]`. Otherwise the floor is `values[i - 1]` (or
`-1` when `i == 0`) and the ceiling is `values[i]` (or `-1` when `i` is past
the end). Do the traversal iteratively.
