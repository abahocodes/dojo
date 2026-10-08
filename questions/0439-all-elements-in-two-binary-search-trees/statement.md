You are given the roots of two binary search trees, `root1` and `root2`.
Return one list holding every value from both trees in **ascending order**.
If a value appears in both trees, it appears once for each occurrence.

Either tree may be empty. Aim to combine the trees in time linear in the total
number of nodes, without a general-purpose sort.

## Example 1

```
root1  = [2, 1, 4]
root2  = [1, 0, 3]
output = [0, 1, 1, 2, 3, 4]
```

## Example 2

```
root1  = [1, null, 8]
root2  = [8, 1]
output = [1, 1, 8, 8]
```

## Constraints

- Each tree has between `0` and `5000` nodes.
- `-10^5 <= node.val <= 10^5`
- Values inside one tree are distinct; both trees are valid BSTs given in
  level order with `null` for a missing child.
