You are given the `root` of a non-empty binary tree. Decide whether the tree is
**complete**:

- every level except possibly the deepest one has all of its possible nodes,
  and
- the nodes on the deepest level are packed to the left, with no gaps between
  them and the left edge.

Equivalently: if you number the positions of a full binary tree level by level
from left to right (root `1`, its children `2` and `3`, and so on), a tree with
`n` nodes is complete exactly when it occupies positions `1` through `n`.

Return `true` if the tree is complete and `false` otherwise.

## Example 1

```
root   = [4, 9, 2, 7, 1, 3]
output = true     # the last level holds 7, 1, 3, packed to the left
```

## Example 2

```
root   = [4, 9, 2, 7, null, 3]
output = false
```

Node `9` is missing its right child, yet `3` appears further right on the same
level, so the last level has a gap.

## Constraints

- The tree has between `1` and `100` nodes.
- `1 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
