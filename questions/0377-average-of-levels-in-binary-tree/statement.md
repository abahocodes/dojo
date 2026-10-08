You are given the `root` of a non-empty binary tree. The root is on level 0,
its children on level 1, their children on level 2, and so on.

For each level, compute the average of the values of the nodes on that level
(their sum divided by how many there are). Return the averages as a list,
starting with the root's level and going down. Answers within `1e-6` of the
exact values are accepted.

## Example 1

```
root   = [3, 8, 2, null, 4, 7]
output = [3.0, 5.0, 5.5]     # (8 + 2) / 2 = 5, (4 + 7) / 2 = 5.5
```

## Example 2

```
root   = [2147483647, 2147483647, 2147483646]
output = [2147483647.0, 2147483646.5]
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-2^31 <= node.val <= 2^31 - 1`
- The tree is given in level order; `null` marks a missing child.
