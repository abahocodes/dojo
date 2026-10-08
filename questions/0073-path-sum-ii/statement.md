You are given the `root` of a binary tree and an integer `target_sum`. A
**root-to-leaf path** starts at the root and walks down to a leaf (a node with
no children).

Return every root-to-leaf path whose values add up to exactly `target_sum`.
Write each path as the list of its values from the root down to the leaf. List
the paths in the **left-to-right order of their leaves**, that is, the order in
which a depth-first traversal that visits left children before right children
reaches those leaves. Return `[]` if no path qualifies.

## Example 1

```
root       = [1, 2, 3, 4, 5, 3, 0, null, null, null, null, null, null, 3]
target_sum = 7
output     = [[1, 2, 4], [1, 3, 3], [1, 3, 0, 3]]
```

## Example 2

```
root       = [-2, null, -3]
target_sum = -5
output     = [[-2, -3]]
```

## Example 3

```
root       = [1, 2]
target_sum = 1
output     = []     # the root alone is not a leaf, so [1] doesn't count
```

## Constraints

- The tree has between `0` and `2000` nodes.
- `-1000 <= node.val <= 1000`
- `-10^6 <= target_sum <= 10^6`
- The tree is given in level order; `null` marks a missing child.
