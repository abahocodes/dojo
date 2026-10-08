# Hints

## Hint 1
Recursively: visit the left subtree, record the node, visit the right subtree.
On a tree that is thousands of levels deep, though, recursion can overflow the
call stack.

## Hint 2
To do it iteratively, simulate the call stack. Before you can record a node you
must finish its entire left subtree, so push nodes while walking left.

## Hint 3
`cur = root`; loop while `cur` or the stack is non-empty: push and go left
while `cur` exists; then pop a node, record its value and set
`cur = node.right`.
