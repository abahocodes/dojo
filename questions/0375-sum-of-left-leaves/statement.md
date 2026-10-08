You are given the `root` of a binary tree. A **left leaf** is a node that has
no children and is the **left** child of its parent. The root has no parent, so
it is never a left leaf, even when it is the only node.

Return the sum of the values of all left leaves. If there are none, return `0`.

## Example 1

```
root   = [8, 3, 10, null, 6, 9, 14]
output = 9      # 9 is the only left leaf; 6 and 14 are right children
```

## Example 2

```
root   = [1, 2, 3, 4, 5, -6]
output = -2     # left leaves 4 and -6; 2 has children, so it is not a leaf
```

## Constraints

- The tree has between `1` and `1000` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
