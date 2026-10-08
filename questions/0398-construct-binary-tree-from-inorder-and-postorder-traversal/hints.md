# Hints

## Hint 1
Which list tells you the root right away? Once you know the root's value,
what does its position in `inorder` tell you about the two subtrees?

## Hint 2
The root is the last element of `postorder`. Everything left of it in
`inorder` is the left subtree and everything right of it is the right subtree,
and the subtree sizes tell you how to cut `postorder` too. A value-to-index map
over `inorder` makes each lookup O(1).

## Hint 3
Read `postorder` from the back: that order is root, right subtree, left
subtree. Keep a stack of nodes on the current path and a pointer `i` that
walks `inorder` from the back. A new value is the right child of the stack top
unless the top equals `inorder[i]`; in that case pop while the top equals
`inorder[i]` (moving `i` left each time) and make the new value the left child
of the last popped node. This avoids recursion on deep trees.
