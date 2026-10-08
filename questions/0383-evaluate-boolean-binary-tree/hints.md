# Hints

## Hint 1
To know the value of an operator node you first need the values of both of its
children. Which traversal order guarantees that?

## Hint 2
Post-order: evaluate the left subtree, then the right subtree, then combine
them with the node's operator. A leaf is its own value (`val == 1`).

## Hint 3
Recursively: `if not node.left: return node.val == 1`; otherwise evaluate both
children and return `l or r` for `2`, `l and r` for `3`. To avoid deep
recursion, simulate it with a stack of `(node, children_done)` pairs and a map
from node to its computed value.
