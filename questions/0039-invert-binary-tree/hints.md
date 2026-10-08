# Hints

## Hint 1
Mirroring the whole tree means mirroring every subtree too. What is the
smallest piece of work you can do at a single node?

## Hint 2
At each node, swap its `left` and `right` pointers. Then the same job remains
for both children. Any traversal order works, as long as every node is visited
exactly once.

## Hint 3
The tree can be thousands of levels deep, so prefer an explicit stack (or a
queue): pop a node, swap its children, push both children if they exist.
Return the original `root`.
