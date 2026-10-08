You are given the `root` of a **full** binary tree (every node has zero or
two children) that encodes a boolean expression:

- a leaf holds `0` (false) or `1` (true);
- a node with children holds `2` (OR) or `3` (AND), and its value is that
  operation applied to the values of its left and right children.

Return the boolean value of the whole expression, that is, the value of the
root.

## Example 1

```
root   = [3, 2, 1, 1, 0]
output = true     # AND(OR(true, false), true)
```

## Example 2

```
root   = [2, 3, 0, 1, 0]
output = false    # OR(AND(true, false), false)
```

## Constraints

- The tree has between `1` and `1000` nodes.
- Leaves hold `0` or `1`; every other node holds `2` or `3` and has exactly two
  children.
- The tree is given in level order; `null` marks a missing child.
