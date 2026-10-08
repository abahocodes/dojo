You are given `root`, the root of a non-empty binary tree.

Return `true` if the tree is its own mirror image around a vertical line
through the root: the left subtree must be the mirror reflection of the right
subtree, with the same shape and the same values in mirrored positions.
Otherwise return `false`.

Trees are written in level order, with `null` marking a missing child.

## Example 1

```
root   = [4, 2, 2, 7, 1, 1, 7]

        4
       / \
      2   2
     / \ / \
    7  1 1  7

output = true
```

## Example 2

```
root   = [4, 2, 2, null, 7, null, 7]

        4
       / \
      2   2
       \   \
        7   7

output = false
```

The values match level by level, but the shapes are not mirror images.

## Constraints

- The tree has between `1` and `1000` nodes.
- `-100 <= node.val <= 100`
