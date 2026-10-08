You are given the `root` of a non-empty binary tree. Look at the **deepest**
level of the tree (the nodes farthest from the root) and return the value of
the **leftmost** node on that level.

The leftmost node of the deepest level is not necessarily a left child: if the
deepest level holds a single node, that node is the answer wherever it hangs.

## Example 1

```
root   = [2, 1, 3]
output = 1
```

## Example 2

```
root   = [1, 2, 3, 4, null, 5, 6, null, null, 7]
output = 7        # 7 is the only node on the deepest level
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-2^31 <= node.val <= 2^31 - 1`
- The tree is given in level order; `null` marks a missing child.
