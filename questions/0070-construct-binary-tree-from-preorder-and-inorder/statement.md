A binary tree with **distinct** values was walked twice:

- `preorder` lists the values in pre-order (node, then its left subtree, then
  its right subtree).
- `inorder` lists the values in in-order (left subtree, then the node, then its
  right subtree).

Rebuild the original tree from these two lists and return its `root`. Because
all values are distinct, exactly one tree matches both lists. If the lists are
empty, return an empty tree (`null` / `None`).

The tree is shown below in level order, with `null` marking a missing child.

## Example 1

```
preorder = [6, 2, 1, 4, 3, 9, 12]
inorder  = [1, 2, 3, 4, 6, 9, 12]
output   = [6, 2, 9, 1, 4, null, 12, null, null, 3]
```

## Example 2

```
preorder = [5, 8, 7]
inorder  = [5, 7, 8]
output   = [5, null, 8, 7]   # 5 is first in inorder, so it has no left subtree
```

## Constraints

- `0 <= preorder.length == inorder.length <= 2000`
- `-3000 <= value <= 3000`, and all values are distinct.
- `inorder` is a permutation of `preorder`, and both come from the same tree.
