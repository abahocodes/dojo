You are given the `root` of a binary search tree whose values are all distinct
(smaller values to the left, larger to the right), and two values `p` and `q`
that both appear in the tree.

Find their **lowest common ancestor**: the deepest node that has both the node
holding `p` and the node holding `q` in its subtree. A node counts as part of
its own subtree, so a node can be the ancestor of itself. Return that node's
value.

## Example 1

```
root   = [50, 30, 70, 20, 40, 60, 80],  p = 20, q = 40
output = 30
```

## Example 2

```
root   = [50, 30, 70, 20, 40, 60, 80],  p = 70, q = 80
output = 70     # 70 is an ancestor of 80 and of itself
```

## Constraints

- The tree has between `2` and `3000` nodes.
- `-10^9 <= node.val <= 10^9`, and all values are distinct.
- `p != q`, and both values are present in the tree.
