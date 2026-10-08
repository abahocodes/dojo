# Hints

## Hint 1
Explore the tree depth-first and keep the values on the current path from the
root. What do you check when you arrive at a leaf?

## Hint 2
Carry the running sum along with the path. At a leaf, if the sum equals
`target_sum`, save a *copy* of the path. Visiting the left child before the
right child produces the paths in the required order.

## Hint 3
Use one shared path list: append a node's value when you enter it and remove
it when you leave (backtracking). With an explicit stack, push a "leave this
node" marker before pushing the children, so the undo happens after both
subtrees are done.
