You are given the `root` of a non-empty binary tree. Number its levels from the
top: the root is on level `0`, its children on level `1`, and so on.

The tree is **even-odd** when both rules hold on every level:

- On an **even-numbered** level, every value is **odd**, and the values read
  from left to right are **strictly increasing**.
- On an **odd-numbered** level, every value is **even**, and the values read
  from left to right are **strictly decreasing**.

Return `true` if the tree is even-odd, otherwise `false`.

## Example 1

```
root   = [1, 10, 4, 3, null, 7, 9, 12, 8]
output = true
```

Level 0 is `[1]` (odd). Level 1 is `[10, 4]` (even, decreasing). Level 2 is
`[3, 7, 9]` (odd, increasing). Level 3 is `[12, 8]` (even, decreasing).

## Example 2

```
root   = [5, 4, 2, 3, 3, 7]
output = false   # level 2 is [3, 3, 7], which is not strictly increasing
```

## Constraints

- The tree has between `1` and `10^5` nodes.
- `1 <= node.val <= 10^6`
- The tree is given in level order; `null` marks a missing child.
