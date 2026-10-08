You are given the `root` of a non-empty binary tree. Imagine each level laid out
as if the tree were a complete binary tree, with empty slots where nodes are
missing. The **width** of a level is the number of slots from its leftmost
node to its rightmost node, inclusive, counting the empty slots in between.

Return the largest width over all levels.

## Example 1

```
root   = [7, 2, 9, null, 4, 1, null, 6, null, null, 3]
output = 4
```

On level 3, node `6` is the left child of `4` and node `3` is the right child
of `1`. Laid out completely, that level has slots
`_ _ 6 _ _ 3 _ _`, so from `6` to `3` spans 4 slots.

## Example 2

```
root   = [1, 2, null, 3]
output = 1      # every level holds a single node
```

## Constraints

- The tree has between `1` and `3000` nodes.
- `-100 <= node.val <= 100`
- The answer fits in a 32-bit signed integer.
- The tree is given in level order; `null` marks a missing child.
