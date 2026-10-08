You are given the roots of two binary trees, `root1` and `root2`. Read the
leaves of a tree (nodes with no children) from the leftmost to the rightmost:
the values you collect form the tree's **leaf sequence**.

Return `true` if both trees have exactly the same leaf sequence (same length,
same values in the same order), and `false` otherwise. Internal nodes and the
shape of the trees do not matter.

## Example 1

```
root1  = [8, 3, 10, 1, 6, null, 14]
root2  = [2, 7, 5, 1, null, 6, 14]
output = true     # both leaf sequences are [1, 6, 14]
```

## Example 2

```
root1  = [1, 2, 3]
root2  = [1, 3, 2]
output = false    # [2, 3] versus [3, 2]: order matters
```

## Constraints

- Each tree has between `1` and `200` nodes.
- `0 <= node.val <= 200`
- Trees are given in level order; `null` marks a missing child.
