# Hints

## Hint 1
If all nodes share one value, that value must be the root's value.

## Hint 2
Visit every node with any traversal and compare it with that value.

## Hint 3
Store `root.val`, walk the tree with a stack (or queue), and return `false` the
moment a node's value differs. If the walk finishes, return `true`.
