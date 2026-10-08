You are given the `root` of a binary tree in which every node holds either
`0` or `1`. Walking from the root down to a leaf and writing the digits you
pass spells a binary number, with the root's digit as the **most significant
bit**. For example, the path `1 -> 0 -> 1` spells `101`, which is `5`.

Return the sum of the numbers spelled by **all** root-to-leaf paths. A leaf is
a node with no children; leading zeros are allowed and do not change a
number's value.

## Example 1

```
root   = [1, 1, 0, 0, null, 1, 1]
output = 16       # paths 110 (6), 101 (5) and 101 (5)
```

## Example 2

```
root   = [0]
output = 0        # a single path spelling 0
```

## Constraints

- The tree has between `1` and `1000` nodes.
- `node.val` is `0` or `1`.
- The answer fits in a signed 32-bit integer.
- The tree is given in level order; `null` marks a missing child.
