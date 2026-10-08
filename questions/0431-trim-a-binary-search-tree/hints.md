# Hints

## Hint 1
If a node's value is below `low`, what can you say about every node in its left
subtree?

## Hint 2
A node below `low` disappears together with its left subtree, and its trimmed
right subtree takes its place (symmetrically for a node above `high`). Nodes in
range keep their position and just trim their children.

## Hint 3
To do it without recursion: first move the root until it is in range (go right
while it is too small, left while it is too large). Then walk down the left
spine: while `node.left` is below `low`, replace it with `node.left.right`,
otherwise step left. Do the mirror image on the right side with `high`.
