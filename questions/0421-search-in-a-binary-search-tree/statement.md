You are given the `root` of a binary search tree whose values are all
different, and an integer `val`. In a binary search tree, every value in a
node's left subtree is smaller than the node's value, and every value in its
right subtree is larger.

Find the node holding `val` and return it. The answer is the whole subtree
hanging from that node. If no node holds `val`, return an empty tree (`null`).

## Example 1

```
root   = [8, 3, 10, 1, 6, null, 14, null, null, 4, 7]
val    = 6
output = [6, 4, 7]     # the node 6 together with its children 4 and 7
```

## Example 2

```
root   = [8, 3, 10, 1, 6, null, 14, null, null, 4, 7]
val    = 5
output = []            # 5 is not in the tree
```

## Constraints

- The tree has between `1` and `5000` nodes.
- `1 <= node.val, val <= 10^7`
- All node values are distinct, and the tree is a valid binary search tree.
