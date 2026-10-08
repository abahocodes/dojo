# Hints

## Hint 1
Don't try to do it in a single walk. Split the answer into its four parts
(root, left edge, leaves, right edge) and produce each one separately.

## Hint 2
The two edges are simple loops down from the root's children: prefer one
side's child, fall back to the other, and stop *before* appending a leaf. The
leaves come from any traversal that visits left subtrees before right
subtrees, such as pre-order.

## Hint 3
Collect the right edge top-down into a temporary list and append it reversed.
Handle the single-node tree first so the root isn't also reported as a leaf,
and skip leaves in both edge loops so no node is counted twice.
