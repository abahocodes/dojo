You are given the `root` of a binary tree (an ordinary binary tree, **not** a
search tree) whose node values are all distinct, plus two different values `p`
and `q` that both appear in the tree.

Find the deepest node that has both `p` and `q` in its subtree, and return that
node's **value**. A node counts as part of its own subtree, so if one of the
two values sits above the other, the upper one is the answer.

## Example 1

```
root   = [6, 2, 9, 0, 4, 7, 11, null, null, 3, 5]
p      = 2
q      = 9
output = 6       # 2 and 9 sit in different subtrees of the root
```

## Example 2

```
root   = [6, 2, 9, 0, 4, 7, 11, null, null, 3, 5]
p      = 5
q      = 4
output = 4       # 5 is a child of 4, and 4 contains itself
```

## Constraints

- The tree has between `2` and `10^5` nodes.
- `-10^9 <= node.val <= 10^9`, and all values are distinct.
- `p != q`, and both values occur in the tree.
- The tree is given in level order; `null` marks a missing child. It may be
  very deep (close to a single chain).
