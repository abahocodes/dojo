You are given the `root` of a binary search tree that may be badly lopsided.
Return a **height-balanced** BST that holds exactly the same values. A tree is
height-balanced when, at every node, the heights of the two subtrees differ by
at most one.

Many balanced trees are possible, so build this specific one: list the values
in ascending order, then for a range of indices `lo..hi` (inclusive) make the
value at index `(lo + hi) // 2` (rounded down) the root, build its left
subtree from `lo..mid-1` and its right subtree from `mid+1..hi`. Start with
the whole range.

## Example 1

```
root   = [1, null, 2, null, 3, null, 4]
output = [2, 1, 3, null, null, null, 4]
```

Sorted values `[1, 2, 3, 4]`: index `(0 + 3) // 2 = 1` gives root `2`; the right
range `[3, 4]` picks `3` with `4` as its right child.

## Example 2

```
root   = [2, 1, 3]
output = [2, 1, 3]     # already balanced and matches the rule
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `1 <= node.val <= 10^5`, and all values are distinct.
- The input is a valid BST, given in level order with `null` for a missing
  child.
