You are given `preorder`, the preorder traversal (node, then left subtree, then
right subtree) of a binary search tree with distinct values. Rebuild the tree
and return its root. A BST with distinct values is uniquely determined by its
preorder traversal.

Aim for O(n) time.

## Example 1

```
preorder = [8, 5, 1, 7, 10, 12]
output   = [8, 5, 10, 1, 7, null, 12]
```

```
        8
      /   \
     5     10
    / \      \
   1   7      12
```

## Example 2

```
preorder = [1, 3]
output   = [1, null, 3]
```

## Constraints

- `1 <= preorder.length <= 10^4`
- `1 <= preorder[i] <= 10^8`, and all values are distinct.
- `preorder` is the preorder traversal of some BST.
- The tree is returned in level order; `null` marks a missing child.
