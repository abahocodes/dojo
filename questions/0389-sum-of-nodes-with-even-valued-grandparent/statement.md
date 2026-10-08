You are given the `root` of a non-empty binary tree. A node's **grandparent** is
the parent of its parent, so the root and the root's children have no
grandparent.

Return the sum of the values of every node whose grandparent exists and has an
**even** value. If there is no such node, return `0`.

## Example 1

```
root   = [4, 3, 6, 5, 2, 1, 9, null, 7, null, null, 8]
output = 25
```

The grandparent `4` is even, so its grandchildren `5`, `2`, `1` and `9` count
(17). The grandparent `6` is even, so its grandchild `8` counts too. Node `7`'s
grandparent is `3`, which is odd. Total: `17 + 8 = 25`.

## Example 2

```
root   = [3, 5, 7, 2, 4]
output = 0      # the only grandparent, 3, is odd
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `1 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.
