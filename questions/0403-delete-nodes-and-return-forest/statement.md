You are given the `root` of a binary tree with distinct node values and a list
`to_delete` of distinct values.

Remove every node whose value is in `to_delete`. Removing a node cuts the links
to its parent and its children, so the tree falls apart into a forest of
smaller trees (nodes that are not deleted keep their other links).

Return the root of every tree left in the forest, each one a complete tree
with its remaining nodes. The roots may be returned in **any order**; if every
node is deleted, return an empty list.

## Example 1

```
root      = [1, 2, 3, 4, 5, 6, 7]
to_delete = [2, 6]
output    = [[1, null, 3, null, 7], [4], [5]]
```

## Example 2

```
root      = [4, 2, null, 1]
to_delete = [4]
output    = [[2, 1]]
```

## Constraints

- The tree has between `1` and `1000` nodes.
- `1 <= node.val <= 1000`, and all values are distinct.
- `0 <= len(to_delete) <= 1000`; its values are distinct and lie in
  `[1, 1000]` (they need not all occur in the tree).
- The tree is given in level order; `null` marks a missing child. Each returned
  tree is printed the same way.
