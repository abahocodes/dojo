You are given the `root` of a binary search tree and an integer `k`. In a
binary search tree every value in a node's left subtree is smaller than the
node's value, and every value in its right subtree is larger. All values are
distinct.

Imagine listing every value in the tree from smallest to largest. Return the
value at position `k` of that list, counting from `1`.

## Example 1

```
root   = [5, 3, 8, 2, 4, 7, 9, 1]
k      = 3
output = 3      # sorted values: 1, 2, 3, 4, 5, 7, 8, 9
```

## Example 2

```
root   = [20, 10, 30, null, 15, 25]
k      = 4
output = 25     # sorted values: 10, 15, 20, 25, 30
```

## Constraints

- The tree has `n` nodes, where `1 <= n <= 2000`.
- `1 <= k <= n`
- `-10^4 <= node.val <= 10^4`, and all values are distinct.
- The tree is a valid binary search tree, given in level order; `null` marks a
  missing child.
