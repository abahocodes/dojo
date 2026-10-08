# Approach: search, then attach

A failed search for `val` stops at a null child. Placing the new node in that
slot keeps the BST ordering, because every comparison made on the way down
holds for the new value too. No existing node moves.

An iterative loop avoids recursion depth problems on skewed trees.

```python
def insert_into_bst(root, val):
    node = TreeNode(val)
    if root is None:
        return node
    cur = root
    while True:
        if val < cur.val:
            if cur.left is None:
                cur.left = node
                return root
            cur = cur.left
        else:
            if cur.right is None:
                cur.right = node
                return root
            cur = cur.right
```

## Complexity

- Time: O(h), the height of the tree.
- Space: O(1) iteratively (O(h) for a recursive version).

## Pitfalls

- Returning the new node instead of the original root when the tree is not
  empty.
- Forgetting the empty-tree case.
- "Balancing" or otherwise restructuring the tree: only the leaf position
  reached by the search is accepted.
