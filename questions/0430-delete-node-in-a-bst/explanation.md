# Approach: locate, then splice

1. Walk down from the root to find the node with `key`, tracking its parent.
   If it is missing, return the root unchanged.
2. **Two children:** walk to the leftmost node of the right subtree (the
   inorder successor), copy its value into the node, and unlink the successor.
   The successor has no left child, so unlinking it means pointing its parent
   at its right child. If the successor is the node's immediate right child,
   that parent link is the node's `right` pointer; otherwise it is the
   successor parent's `left` pointer.
3. **Zero or one child:** replace the node by its only child (or null). If the
   node was the root, that child becomes the new root.

```python
def delete_node(root, key):
    parent, node = None, root
    while node is not None and node.val != key:
        parent = node
        node = node.left if key < node.val else node.right
    if node is None:
        return root

    if node.left is not None and node.right is not None:
        succ_parent, succ = node, node.right
        while succ.left is not None:
            succ_parent, succ = succ, succ.left
        node.val = succ.val
        if succ_parent is node:
            succ_parent.right = succ.right
        else:
            succ_parent.left = succ.right
        return root

    child = node.left if node.left is not None else node.right
    if parent is None:
        return child
    if parent.left is node:
        parent.left = child
    else:
        parent.right = child
    return root
```

The common recursive version (`root.right = delete_node(root.right, succ.val)`)
yields the same tree, but recursion on a degenerate chain can be very deep.

## Complexity

- Time: O(h): one descent to find the node plus one to find the successor.
- Space: O(1).

## Pitfalls

- Using the inorder *predecessor* instead of the successor: also a valid BST,
  but not the tree this problem asks for.
- Forgetting that the successor can have a right child that must be kept.
- Mishandling the successor when it is the node's direct right child.
- Deleting the root: the returned root must change when the root has fewer
  than two children.
