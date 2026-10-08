You are given the roots of two binary trees, `root` and `sub_root`. Return
`true` if some node of `root`, taken together with **all** of its descendants,
is identical to `sub_root`: the same shape, with the same value at every
position. Otherwise return `false`.

A tree counts as a subtree of itself.

## Example 1

```
root     = [4, 7, 2, 1, 9]
sub_root = [7, 1, 9]
output   = true
```

## Example 2

```
root     = [4, 7, 2, 1, 9, null, null, null, null, 6]
sub_root = [7, 1, 9]
output   = false   # the 7 in root also has 6 below its 9
```

## Constraints

- `root` has between `1` and `2000` nodes.
- `sub_root` has between `1` and `1000` nodes.
- `-10^4 <= node.val <= 10^4`
- Both trees are given in level order; `null` marks a missing child.
