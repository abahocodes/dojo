# Hints

## Hint 1
Whether a node is good depends only on the values *above* it. What single
number about the path from the root summarises everything you need?

## Hint 2
Carry the maximum value seen so far on the path down the tree. A node is good
exactly when its value is at least that maximum.

## Hint 3
Run a DFS (an explicit stack of `(node, max_so_far)` pairs avoids deep
recursion). Count the node if `node.val >= max_so_far`, then push each child
with `max(max_so_far, node.val)`.
