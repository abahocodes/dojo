You are given the roots of two binary trees, `root1` and `root2`. Lay one tree
on top of the other so that their roots line up, and build a single merged
tree position by position:

- where **both** trees have a node, the merged node's value is the sum of the
  two values;
- where only **one** tree has a node, the merged tree uses that node (with its
  whole subtree below, merged against nothing);
- where neither has a node, the merged tree has none.

Return the root of the merged tree. You may reuse or modify the input nodes.
If both trees are empty, return an empty tree.

## Example 1

```
root1  = [3, 1, 4, 2]
root2  = [5, 6, 2, null, 7, null, 8]
output = [8, 7, 6, 2, 7, null, 8]
```

The roots give `3 + 5`, the left children `1 + 6`, the right children `4 + 2`.
Node `2` exists only in `root1`, and nodes `7` and `8` only in `root2`.

## Example 2

```
root1  = []
root2  = [1, -2, null, 4]
output = [1, -2, null, 4]     # nothing to add, so the merged tree is root2
```

## Constraints

- Each tree has between `0` and `2000` nodes.
- `-10^4 <= node.val <= 10^4`
- Trees are given and returned in level order; `null` marks a missing child.
