You are given the `root` of a binary tree. Decide whether it is a valid
**binary search tree**, which here means that for every node:

- every value in its left subtree is strictly smaller than the node's value, and
- every value in its right subtree is strictly larger than the node's value.

Duplicate values therefore make a tree invalid. Return `true` or `false`.

## Example 1

```
root   = [20, 10, 30, 5, 15, 25, 35]
output = true
```

## Example 2

```
root   = [20, 10, 30, null, 25]
output = false     # 25 is in 20's left subtree but larger than 20
```

## Constraints

- The tree has between `1` and `3000` nodes.
- `-2^31 <= node.val <= 2^31 - 1`
- The tree is given in level order; `null` marks a missing child.
