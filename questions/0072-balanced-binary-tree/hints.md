# Hints

## Hint 1
Checking only the root is not enough: the condition must hold at every node.
What do you need to know about each node's two subtrees?

## Hint 2
Computing a fresh height for every node repeats work and is O(n^2) on a
skewed tree. Can one bottom-up pass give you each node's height from its
children's heights?

## Hint 3
Process nodes in post-order (children first). For each node, read the two
child heights, return `false` if they differ by more than 1, and otherwise
store `1 + max(left, right)`. An explicit stack avoids deep recursion.
