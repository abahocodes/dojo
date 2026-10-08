You are given an array `preorder` of distinct integers. Decide whether it could
be the **preorder traversal** (node, then left subtree, then right subtree) of
some binary search tree. Return `true` if such a tree exists and `false`
otherwise.

In the BST, every value in a node's left subtree is smaller than the node and
every value in its right subtree is larger.

Aim for O(n) time.

## Example 1

```
preorder = [8, 4, 2, 6, 10, 12]
output   = true      # root 8, left subtree 4 -> (2, 6), right subtree 10 -> 12
```

## Example 2

```
preorder = [8, 4, 10, 2, 6]
output   = false     # after moving right to 10, the value 2 (< 8) cannot appear
```

## Constraints

- `1 <= preorder.length <= 10^4`
- `1 <= preorder[i] <= 10^4`
- All values in `preorder` are distinct.
