# Hints

## Hint 1
The middle element of the list is the root, everything before it forms the left
subtree and everything after it the right subtree. Copying the list into an
array makes this easy; can you avoid the copy?

## Hint 2
An inorder traversal of the finished tree visits the values in exactly the
list's order. If you build the tree in inorder, you can consume the list one
node at a time.

## Hint 3
Count the nodes first. Then `build(lo, hi)`: pick `mid = (lo + hi + 1) // 2`,
build the left subtree from `lo .. mid-1` (this consumes those list nodes), make
the root from the current list node and advance, then build the right subtree
from `mid+1 .. hi`.
