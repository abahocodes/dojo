# Hints

## Hint 1
The root is `preorder[0]` (and `postorder[-1]`). If the root has any children,
the value right after it in `preorder` is the root of its left subtree.

## Hint 2
Find that left-child value in `postorder`. The left subtree ends exactly there
in `postorder`, so its position tells you the left subtree's size, and the
size tells you where to cut both lists. Recurse on each part.

## Hint 3
There is also a one-pass version. Walk `preorder` while keeping a stack of the
current path and a pointer `j` into `postorder`. Before attaching a new value,
pop the stack while its top equals `postorder[j]` (each such node is
complete), moving `j` forward. Then attach the new node as the top's left
child if that slot is free, otherwise as its right child, and push it.
