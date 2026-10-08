# Hints

## Hint 1
Rebalancing by rotations is hard. Is there a traversal that hands you the
values already sorted?

## Hint 2
An in-order traversal of a BST yields its values in ascending order. From a
sorted array, how do you build a balanced BST?

## Hint 3
Collect the values with an iterative in-order traversal (the input can be a
10^4-deep chain). Then `build(lo, hi)`: if `lo > hi` return null; otherwise
`mid = (lo + hi) // 2`, create a node for `values[mid]` and attach
`build(lo, mid - 1)` and `build(mid + 1, hi)`. This recursion is only
O(log n) deep.
