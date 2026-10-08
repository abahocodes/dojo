A binary tree with distinct node values was traversed twice:

- `inorder` lists the values as left subtree, node, right subtree;
- `postorder` lists them as left subtree, right subtree, node.

Rebuild the tree from these two lists and return its root. Because the values
are distinct, exactly one tree produces both lists.

## Example 1

```
inorder   = [4, 2, 5, 1, 3]
postorder = [4, 5, 2, 3, 1]
output    = [1, 2, 3, 4, 5]
```

## Example 2

```
inorder   = [-1]
postorder = [-1]
output    = [-1]
```

## Constraints

- `1 <= len(inorder) == len(postorder) <= 3000`
- `-3000 <= value <= 3000`, and all values are distinct.
- Both lists hold the same values and come from one binary tree.
- The returned tree is printed in level order, with `null` for a missing child.
