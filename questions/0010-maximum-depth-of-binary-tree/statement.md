You are given the `root` of a binary tree. Its **depth** is the number of nodes on
the longest path that starts at the root and walks downward to a leaf.

Return the depth of the tree. An empty tree has depth `0`.

## Example 1

```
root   = [8, 4, 12, null, 6, 10, 14, null, null, null, null, 13]
output = 4      # longest path: 8 -> 12 -> 14 -> 13
```

## Example 2

```
root   = [2, null, 5]
output = 2      # 2 -> 5
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-10^4 <= node.val <= 10^4`
- The tree is given in level order; `null` marks a missing child.
