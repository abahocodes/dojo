You are given the `root` of a binary tree. A node is **good** if no node on the
path from the root down to it (the node itself included) has a value strictly
greater than its own. In other words, the node holds the maximum of its path,
and ties are allowed. The root is always good.

Return the number of good nodes in the tree.

## Example 1

```
root   = [2, 5, 1, 4, 6, null, 3]
output = 4      # good: 2, 5, 6 (path 2 -> 5 -> 6) and 3 (path 2 -> 1 -> 3)
```

## Example 2

```
root   = [-1, -1, -2]
output = 2      # the second -1 ties the root, so it is good; -2 is not
```

## Constraints

- The tree has between `1` and `3000` nodes.
- `-10^4 <= node.val <= 10^4`
- The tree is given in level order; `null` marks a missing child.
