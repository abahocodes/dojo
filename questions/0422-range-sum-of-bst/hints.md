# Hints

## Hint 1
Visiting every node and adding the in-range values is correct. How can the BST
ordering let you skip whole subtrees?

## Hint 2
If a node's value is below `low`, then every value in its left subtree is lower
still, so none of them can count. The mirror argument applies above `high`.

## Hint 3
Use a stack. Pop a node. If `node.val < low`, push only its right child. If
`node.val > high`, push only its left child. Otherwise add the value and push
both children.
