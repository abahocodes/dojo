A binary tree whose node values are the numbers `1..n` (each used once) was
traversed twice:

- `preorder` lists the values as node, left subtree, right subtree;
- `postorder` lists them as left subtree, right subtree, node.

Rebuild a tree that produces both lists and return its root.

These two traversals alone can't tell whether a lone child hangs on the left or
the right, so follow this rule: **whenever a node has exactly one child, that
child is the left child.** With this rule the answer is unique.

## Example 1

```
preorder  = [1, 2, 4, 5, 3, 6]
postorder = [4, 5, 2, 6, 3, 1]
output    = [1, 2, 3, 4, 5, 6]
```

## Example 2

```
preorder  = [1, 2, 3]
postorder = [3, 2, 1]
output    = [1, 2, null, 3]    # each lone child goes on the left
```

## Constraints

- `1 <= len(preorder) == len(postorder) <= 30`
- The values are a permutation of `1..n`.
- Both lists come from the same binary tree.
- The returned tree is printed in level order, with `null` for a missing child.
