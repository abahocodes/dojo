You are given the `root` of a non-empty binary tree. The **subtree** of a node
consists of the node itself and all of its descendants. The **average** of a
subtree is the sum of its values divided by the number of nodes in it,
**rounded down** to an integer.

Return how many nodes have a value equal to the average of their own subtree.

## Example 1

```
root   = [5, 9, 3, 2, 7, null, 3]
output = 4
```

The leaves `2`, `7` and the lower `3` each equal their own one-node average.
The upper `3` has subtree `{3, 3}` with average `3`, so it counts. Node `9` has
average `(9 + 2 + 7) / 3 = 6`, and the root has `29 / 6 = 4` (rounded down);
neither matches.

## Example 2

```
root   = [7]
output = 1
```

## Constraints

- The tree has between `1` and `1000` nodes.
- `0 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
