You are given the `root` of a binary tree with at least two nodes. Node `a` is
an **ancestor** of node `b` if `a` lies on the path from the root down to `b`
and `a != b` (a parent, a grandparent, and so on).

Over every pair `(a, b)` where `a` is an ancestor of `b`, find the largest
value of `|a.val - b.val|` and return it.

## Example 1

```
root   = [6, 11, 4, 2, 15, null, 9, null, null, 13, 20]
output = 14
```

The root `6` is an ancestor of `20`, giving `|6 - 20| = 14`. No other
ancestor/descendant pair is further apart.

## Example 2

```
root   = [5, null, 8, null, 3, 1]
output = 7     # 8 is an ancestor of 1
```

## Constraints

- The tree has between `2` and `5000` nodes.
- `0 <= node.val <= 10^5`
- The tree is given in level order; `null` marks a missing child.
