You are given the `root` of a non-empty binary tree with distinct values. The
**depth** of a node is its distance (in edges) from the root, and the
**deepest nodes** are the ones whose depth is the largest in the tree.

Return the root of the **smallest subtree** that contains every deepest node.
A subtree means a node together with all of its descendants, and "smallest"
means the one rooted lowest in the tree. In other words, return the lowest
common ancestor of all the deepest nodes. If there is only one deepest node,
the answer is that node itself.

The returned subtree is shown in level order.

## Example 1

```
root   = [8, 4, 9, 1, 6, null, 12, null, null, 5, 7]
output = [6, 5, 7]
```

The deepest nodes are `5` and `7` (depth 3). The lowest node that has both of
them in its subtree is `6`.

## Example 2

```
root   = [10, 20, 30, null, 40, null, 50, 60]
output = [60]     # 60 is the only node at depth 3
```

## Constraints

- The tree has between `1` and `500` nodes.
- `0 <= node.val <= 500`, and all values are distinct.
- The tree is given in level order; `null` marks a missing child.
