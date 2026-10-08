You are given the `root` of a non-empty binary tree. Call the tree
**univalued** when every one of its nodes holds the same value.

Return `true` if the tree is univalued and `false` otherwise.

## Example 1

```
root   = [6, 6, 6, 6, null, null, 6]
output = true
```

## Example 2

```
root   = [3, 3, 3, null, 3, 4]
output = false     # one node deep in the tree holds 4
```

## Constraints

- The tree has between `1` and `100` nodes.
- `0 <= node.val <= 99`
- The tree is given in level order; `null` marks a missing child.
