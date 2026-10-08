You are given the `root` of a binary tree. A **path** connects two nodes by
moving along parent-child links without visiting any node twice; its length is
the number of links (edges) it uses.

Return the **diameter** of the tree: the length of the longest path between any
two nodes. The path may or may not pass through the root. A tree with a single
node has diameter `0`.

## Example 1

```
root   = [7, 3, 9, 1]
output = 3      # 1 -> 3 -> 7 -> 9
```

## Example 2

```
root   = [1, 2, null, 3, 4, 5, null, null, 6]
output = 4      # 5 -> 3 -> 2 -> 4 -> 6 (it skips the root)
```

## Constraints

- The tree has between `1` and `3000` nodes.
- `-100 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.
