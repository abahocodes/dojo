# Hints

## Hint 1
You do not need to look at every node. Comparing `val` with the current node's
value tells you which side the target must be on, if it is anywhere.

## Hint 2
Start at the root. If the value matches, you are done. If `val` is smaller, only
the left subtree can contain it. If it is larger, only the right subtree can.

## Hint 3
A loop is enough: `while node and node.val != val: node = node.left if
val < node.val else node.right`, then return `node`. It is `None` when the
search falls off the tree.
