# Hints

## Hint 1
Every path has a unique highest node (the one closest to the root). Fix that
"top" node: what is the best path that bends at it?

## Hint 2
The best path topped at a node is its value plus the best downward chain into
the left child plus the best downward chain into the right child, where a
chain that would lower the sum is simply dropped (counted as 0).

## Hint 3
Process nodes children-first. For each node compute
`down = node.val + max(0, down(left), down(right))` (a chain may use only one
side) and update the answer with
`node.val + max(0, down(left)) + max(0, down(right))`. Start the answer at the
root's value, not 0, so all-negative trees work.
