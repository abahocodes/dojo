You are given the `root` of a binary tree. A **leaf** is a node with no
children. The depth of a leaf counts the nodes on the path from the root down to
it, both ends included, so the root alone has depth `1`.

Return the smallest depth of any leaf in the tree. An empty tree has no nodes
and its answer is `0`.

## Example 1

```
root   = [4, 7, 1, null, null, 6, 9]
output = 2      # 7 is a leaf two nodes below the top: 4 -> 7
```

## Example 2

```
root   = [2, null, 3, null, 4, null, 5]
output = 4      # the root has a child, so it is not a leaf; the only leaf is 5
```

## Constraints

- The tree has between `0` and `10^5` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
