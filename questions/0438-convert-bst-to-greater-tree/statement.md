You are given the `root` of a binary search tree with distinct values. Rewrite
the tree in place so that every node's new value is its original value **plus
the sum of every original value in the tree that is greater than it**.

Keep the shape of the tree unchanged and return its root. An empty tree stays
empty.

## Example 1

```
root   = [4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8]
output = [30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]
```

The largest value `8` stays `8`; `7` becomes `7 + 8 = 15`; `6` becomes
`6 + 7 + 8 = 21`; and so on down to `0`, which becomes the sum of all values,
`36`.

## Example 2

```
root   = [0, null, 1]
output = [1, null, 1]
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-10^4 <= node.val <= 10^4`, and all values are distinct.
- The input is a valid BST, given in level order with `null` for a missing
  child.
