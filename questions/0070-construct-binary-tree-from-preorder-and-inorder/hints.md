# Hints

## Hint 1
Which value is the root? Look at the first element of `preorder`.

## Hint 2
Find the root in `inorder`: everything to its left belongs to the left
subtree and everything to its right to the right subtree. The sizes of those
two parts tell you how to split the rest of `preorder`. A hash map from value
to inorder index makes each lookup O(1).

## Hint 3
Recursing on the two halves works, but a very deep tree can overflow the call
stack. Iteratively: walk `preorder` keeping a stack of nodes whose right child
is still open, and a pointer `j` into `inorder`. A new value is the left child
of the stack top unless the top equals `inorder[j]`; in that case pop while the
top equals `inorder[j]` (advancing `j`), and the new value is the right child of
the last node popped.
