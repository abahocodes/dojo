# Hints

## Hint 1
A node with exactly one child is not a leaf. What goes wrong if you take
`1 + min(depth(left), depth(right))` and one side is empty?

## Hint 2
You want the leaf closest to the root. Which traversal visits nodes in order of
their distance from the root, so the first leaf it meets is the answer?

## Hint 3
Run a breadth-first search that carries each node's depth. The first node you
dequeue with no left and no right child is the shallowest leaf: return its
depth right away. Return `0` up front if the tree is empty.
