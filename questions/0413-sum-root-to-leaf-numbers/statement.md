Every node of a binary tree holds a single digit from `0` to `9`. Reading the
digits along a path from the root down to a leaf, root first, gives a decimal
number: the path `5 -> 0 -> 3` spells `503`. Leading zeros are allowed and
simply vanish (`0 -> 4` spells `4`).

Return the sum of the numbers spelled by **all** root-to-leaf paths. A leaf is
a node with no children.

## Example 1

```
root   = [4, 2, 7]
output = 89          # 42 + 47
```

## Example 2

```
root   = [3, 1, 5, 8, null, 0, 2]
output = 1020        # 318 + 350 + 352
```

## Constraints

- The tree has between `1` and `1000` nodes.
- `0 <= node.val <= 9`
- The tree has at most `10` levels (a depth of at most `9` edges).
- The answer is guaranteed to fit in a 32-bit signed integer.
