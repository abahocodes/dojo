# Hints

## Hint 1
Comparing every node with every one of its ancestors is O(n * h), which is
O(n^2) on a path-shaped tree. Fix a node `b`: which of its ancestors can give
the largest `|a.val - b.val|`?

## Hint 2
Only the smallest and the largest ancestor value matter. So as you walk down
from the root, carry the minimum and maximum value seen on the current path.

## Hint 3
Push `(node, lo, hi)` frames on a stack (or pass them through a DFS). At each
node, fold its value into `lo` and `hi` and update the answer with `hi - lo`:
that is the widest gap between any two nodes on the root-to-node path, and any
two nodes on such a path are ancestor and descendant.
