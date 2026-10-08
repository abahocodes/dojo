You are given `n` small binary search trees in the list `trees`. Each tree has
at most 3 nodes: a root plus at most a left child and a right child (so every
child is a leaf). No two trees have the same root value.

A **merge** works like this:

1. Pick two different trees `A` and `B` such that some leaf of `A` holds the
   same value as the root of `B`.
2. Replace that leaf of `A` with the whole tree `B`.
3. Remove `B` from the list.

After exactly `n - 1` merges a single tree is left. Return its root if it is a
valid binary search tree, where every node's value is **strictly** greater than
all values in its left subtree and **strictly** smaller than all values in its
right subtree. If no sequence of merges leads to a valid BST, return an empty
tree. When a valid result exists, it is unique.

Trees are written in level order, with `null` marking a missing child.

## Example 1

```
trees  = [[4, 2, 7], [2, 1, 3], [7, 6, 9]]
output = [4, 2, 7, 1, 3, 6, 9]
```

Merge `[2, 1, 3]` into the leaf `2` and `[7, 6, 9]` into the leaf `7`.

## Example 2

```
trees  = [[8, 3, 12], [3, 1, 9], [12, 10]]
output = []
```

The only possible final tree puts `9` in the left subtree of `8`, so it is not
a valid BST.

## Constraints

- `1 <= n <= 5 * 10^4`
- Each tree has 1 to 3 nodes; children of a root are leaves.
- `1 <= node.val <= 5 * 10^4`
- Every tree is itself a valid BST, and all root values are distinct.
