You are given the `root` of a **complete** binary tree: every level is completely
filled except possibly the last, and the nodes on the last level are packed as
far to the left as possible.

Return the number of nodes in the tree. An empty tree has `0` nodes.

Visiting every node works, but try to use the shape of the tree to do better
than O(n): an O(log^2 n) solution exists.

## Example 1

```
root   = [1, 2, 3, 4, 5, 6]
output = 6
```

## Example 2

```
root   = []
output = 0
```

## Constraints

- The tree has between `0` and `5 * 10^4` nodes.
- `0 <= node.val <= 5 * 10^4`
- The tree is complete.
- The tree is given in level order; `null` marks a missing child.
