You are given the `root` of a binary tree and an integer `target_sum`. A
**root-to-leaf path** starts at the root and follows child links down to a
**leaf**, a node with no children.

Return `true` if the values along at least one root-to-leaf path add up to
exactly `target_sum`, and `false` otherwise. A path must end at a leaf: a
running total that hits `target_sum` at an inner node does not count. An empty
tree has no paths, so the answer is `false` for every `target_sum`.

## Example 1

```
root       = [6, 2, 9, 1, 4, null, 3]
target_sum = 12
output     = true      # 6 + 2 + 4 = 12
```

## Example 2

```
root       = [5, 3, null, 1, 2]
target_sum = 8
output     = false     # 5 + 3 = 8, but 3 is not a leaf; the leaf paths sum to 9 and 10
```

## Constraints

- The tree has between `0` and `5000` nodes.
- `-1000 <= node.val <= 1000`
- `-1000 <= target_sum <= 1000`
- The tree is given in level order; `null` marks a missing child.
