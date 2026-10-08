You are given the `root` of a binary search tree with distinct values and an
integer `p` that is stored in the tree. Return the **inorder successor** of
`p`: the smallest value in the tree that is strictly greater than `p`. If `p`
is the largest value in the tree, return `-1`.

Aim for O(h) time, where `h` is the height of the tree.

## Example 1

```
root   = [20, 10, 30, 5, 15, 25, 35]
p      = 15
output = 20
```

```
          20
        /    \
      10      30
     /  \    /  \
    5   15  25  35
```

`15` has no right subtree, so its successor is the nearest ancestor it sits to
the left of: `20`.

## Example 2

```
root   = [4, 2, 6, 1, 3]
p      = 6
output = -1
```

`6` is the largest value, so it has no successor.

## Constraints

- The tree has between `1` and `10^4` nodes.
- `0 <= node.val <= 10^5`, and all values are distinct.
- `p` is the value of some node in the tree.
- Trees are given (and returned) in level order; `null` marks a missing child.
