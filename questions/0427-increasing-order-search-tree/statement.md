You are given the `root` of a binary search tree with distinct values.
Rearrange it into a chain where every node has **no left child** and at most
one right child. Reading the chain from the top must give the values in
increasing order, so the smallest value becomes the new root.

Return the root of the rearranged tree. You may reuse the existing nodes.

## Example 1

```
root   = [5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]
output = [1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
```

## Example 2

```
root   = [5, 1, 7]
output = [1, null, 5, null, 7]
```

## Constraints

- The tree has between `1` and `100` nodes.
- `0 <= node.val <= 1000`, and all values are distinct.
