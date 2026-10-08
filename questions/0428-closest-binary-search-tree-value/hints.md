# Hints

## Hint 1
If the target is smaller than the current node's value, can anything in the
node's right subtree be closer than the node itself?

## Hint 2
The closest value is either the largest value not above the target or the
smallest value not below it. Both of those lie on the path you would follow
when searching for the target in the tree.

## Hint 3
Walk down from the root: at each node, update the best answer if this node is
strictly closer, or equally close but smaller. Then go left when
`target < node.val`, right when `target > node.val`, and stop on an exact
match.
