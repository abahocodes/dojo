A preorder walk of a binary tree wrote each node as some dashes followed by the
node's value, with nothing in between entries. The number of dashes is the
node's depth: the root has `0`, its children `1`, their children `2`, and so on.
Whenever a node has exactly one child, that child is its **left** child.

Given the written string `traversal`, rebuild the tree and return its root.

## Example 1

```
traversal = "3-8--1--4-6--2--9"
output    = [3, 8, 6, 1, 4, 2, 9]
```

## Example 2

```
traversal = "4-11--15---8-6--2---30"
output    = [4, 11, 6, 15, null, 2, null, 8, null, 30]
```

`11` has the single child `15`, which therefore sits on the left; likewise
for `6` and `2`, and for `15` and `8`.

## Constraints

- The tree has between `1` and `1000` nodes.
- `1 <= node.val <= 10^9`
- `traversal` is a valid encoding of a tree that follows the
  only-child-is-left rule.
