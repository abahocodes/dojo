You are given the `root` of a non-empty binary tree. Walk around the outside of
the tree **anticlockwise**, starting at the root, and return the values you
pass. The walk is made of four parts, concatenated in this order:

1. **The root.**
2. **The left edge, top-down.** Start at the root's left child and keep
   stepping down: to the left child if there is one, otherwise to the right
   child. Stop when you reach a leaf. Every node on this path *except the
   final leaf* is on the left edge. If the root has no left child, the left
   edge is empty.
3. **Every leaf of the tree, from left to right.**
4. **The right edge, bottom-up.** The mirror image of the left edge: start at
   the root's right child and step to the right child if there is one,
   otherwise to the left child, stopping at a leaf. Every node on this path
   except the final leaf is on the right edge, and these nodes are listed from
   the bottom up. If the root has no right child, the right edge is empty.

Each node appears at most once. The root is never treated as a leaf: a tree
with a single node returns just `[root.val]`.

## Example 1

```
root   = [1, 2, 3, 4, 5, 6, null, null, null, 7, 8, 9, 10]
output = [1, 2, 4, 7, 8, 9, 10, 6, 3]
```

The left edge is `2` (its left child `4` is a leaf). The leaves are
`4, 7, 8, 9, 10`. The right edge walks `3 -> 6` (3 has no right child, so the
walk falls back to its left child) and stops at the leaf `10`, so it
contributes `6, 3` bottom-up.

## Example 2

```
root   = [1, null, 2, 3, 4]
output = [1, 3, 4, 2]     # no left child, so the left edge is empty
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
