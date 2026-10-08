# Hints

## Hint 1
Look at a node's two subtrees. If one is strictly taller than the other, where
must all the deepest nodes of this node's subtree be?

## Hint 2
If the left subtree is taller, the answer for this node is the answer for its
left child (and the same on the right). If both subtrees have the same height,
the deepest nodes are split between them, so the node itself is the answer.

## Hint 3
Write a post-order DFS that returns a pair `(height, answer)`: `(0, None)` for
an empty subtree, otherwise compare the children's heights and pass up the
taller side's answer with `height + 1`, or `(height + 1, node)` on a tie. The
answer for the whole tree is the second item at the root.
