# Hints

## Hint 1
First find the node and remember its parent. The work you do next depends only
on how many children the node has.

## Hint 2
Zero or one child: hook the parent (or the root pointer) directly to that child
(or to null). Two children: the inorder successor is the leftmost node of the
right subtree.

## Hint 3
The successor never has a left child, so deleting it is always the zero- or
one-child case: copy its value into the target node, then point the
successor's parent at the successor's right child. Watch out for the case where
the successor is the target's own right child.
