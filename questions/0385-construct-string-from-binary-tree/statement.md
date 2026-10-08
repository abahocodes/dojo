You are given the `root` of a binary tree. Serialize it into a string in
pre-order, writing each node as its value followed by its children in
parentheses: `value(left)(right)`. Then drop the parentheses that carry no
information:

- a node with **no children** is written as just its value;
- when the **right** child is missing, its `()` is omitted;
- when the **left** child is missing but the right one exists, keep the empty
  `()` for the left child, so the right child is not mistaken for a left one.

Return the resulting string.

## Example 1

```
root   = [1, 2, 3, null, 4]
output = "1(2()(4))(3)"   # node 2 has only a right child, so "()" stays
```

## Example 2

```
root   = [1, 2, 3, 4]
output = "1(2(4))(3)"     # node 2 has only a left child: no "()" for the right
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
