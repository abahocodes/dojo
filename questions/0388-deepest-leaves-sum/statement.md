You are given the `root` of a non-empty binary tree. The nodes that sit on the
**lowest level** of the tree (the level farthest from the root) are its deepest
leaves.

Return the sum of the values of all nodes on that lowest level.

## Example 1

```
root   = [4, 7, 2, 9, null, 6, 8, null, null, 3, 5]
output = 8      # the lowest level is level 3, holding 3 and 5
```

Node `9` is a leaf too, but it sits on level 2, so it does not count.

## Example 2

```
root   = [12]
output = 12     # the root alone forms the lowest level
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `1 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.
