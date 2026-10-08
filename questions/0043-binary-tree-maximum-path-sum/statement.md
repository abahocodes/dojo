A **path** in a binary tree is a sequence of distinct nodes in which every two
consecutive nodes are joined by an edge. A path contains at least one node,
does not have to pass through the root, and may go up through a node and back
down into its other subtree, but it never visits a node twice.

Given the `root` of a non-empty binary tree, return the largest possible sum of
the values on a path.

## Example 1

```
root   = [2, -1, 3, 4]
output = 8         # 4 -> -1 -> 2 -> 3
```

## Example 2

```
root   = [-5, 6, -8, null, null, 7, 9]
output = 9         # the single node 9; any path joining it to 6 costs too much
```

## Constraints

- The tree has between `1` and `3000` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
