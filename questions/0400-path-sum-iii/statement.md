You are given the `root` of a binary tree and an integer `target_sum`.

A **downward path** starts at any node and repeatedly steps from a node to one
of its children, stopping at any node (a single node is a path too). It does
not have to begin at the root or end at a leaf.

Return how many downward paths have node values adding up to exactly
`target_sum`. Two paths are different if they have different start or end
nodes.

## Example 1

```
root       = [4, 3, -1, 2, 1, null, 4, null, null, -2]
target_sum = 5
output     = 1       # only 3 -> 2
```

## Example 2

```
root       = [1, -1, 1, 1, null, null, -1]
target_sum = 0
output     = 3       # 1 -> -1 (left), -1 -> 1, and 1 -> -1 on the right side
```

## Constraints

- The tree has between `0` and `1000` nodes.
- `-10^9 <= node.val <= 10^9`
- `-1000 <= target_sum <= 1000`
- Path sums can exceed the 32-bit range; use 64-bit arithmetic for them.
- The tree is given in level order; `null` marks a missing child.
