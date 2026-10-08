You are given the roots `p` and `q` of two binary trees. Decide whether the two
trees are identical: they must have exactly the same shape, and every pair of
nodes in matching positions must hold the same value.

Return `true` if they are identical and `false` otherwise. Two empty trees are
identical.

## Example 1

```
p      = [6, 2, 9, null, 4]
q      = [6, 2, 9, null, 4]
output = true
```

## Example 2

```
p      = [3, 7]
q      = [3, null, 7]
output = false     # 7 is a left child in p but a right child in q
```

## Constraints

- Each tree has between `0` and `2000` nodes.
- `-10000 <= node.val <= 10000`
- The trees are given in level order; `null` marks a missing child.
