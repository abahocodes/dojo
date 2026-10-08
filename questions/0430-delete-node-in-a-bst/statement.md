You are given the `root` of a binary search tree with distinct values and an
integer `key`. Remove the node holding `key` (if there is one) and return the
root of the resulting tree. If `key` is not in the tree, return the tree
unchanged.

Several valid BSTs could result from a deletion, so use exactly this rule:

- A node with **no children** is simply removed.
- A node with **one child** is replaced by that child (the child's whole
  subtree moves up into its place).
- A node with **two children** keeps its position but takes the value of its
  **inorder successor** (the smallest value in its right subtree); that
  successor node is then deleted from the right subtree using the same rule.

## Example 1

```
root   = [50, 30, 70, 20, 40, 60, 80]
key    = 50
output = [60, 30, 70, 20, 40, null, 80]
```

```
         50                    60
       /    \                /    \
     30      70     ->      30      70
    /  \    /  \           /  \       \
   20  40  60  80         20  40       80
```

`50` has two children, so it takes its successor's value `60`, and the leaf
`60` is removed.

## Example 2

```
root   = [9, 4, 15, null, 6, 12]
key    = 4
output = [9, 6, 15, null, null, 12]
```

`4` has only a right child, so `6` moves up into its place.

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-10^5 <= node.val, key <= 10^5`
- All values in the tree are distinct.
- Trees are given (and returned) in level order; `null` marks a missing child.
