# Hints

## Hint 1
If you already knew the depth of the left subtree and of the right subtree,
how would you get the depth of the whole tree?

## Hint 2
The depth of a node is `1 + max(depth(left), depth(right))`, and the depth of
an empty subtree is `0`. That is a complete recursive definition.

## Hint 3
Alternatively, walk the tree one level at a time (BFS) and count how many levels
you process before the queue runs dry. This avoids deep recursion on a skewed tree.
