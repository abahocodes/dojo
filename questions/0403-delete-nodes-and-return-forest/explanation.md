# Approach: one traversal with a "parent is gone" flag

A surviving node starts a new tree exactly when it has no surviving parent:
it is the original root, or its parent is being deleted. So traverse the tree
once, passing that flag down:

- If the node survives and the flag is set, it is a forest root.
- Its children get the flag "this node is deleted".
- Any child that is itself being deleted is unlinked (`child = None`), so the
  surviving trees no longer reach deleted nodes.

The child is pushed onto the stack before its link is cut, so deleted nodes
are still visited and can hand their own children the "parent is gone" flag.

```python
def del_nodes(root, to_delete):
    doomed = set(to_delete)
    forest = []
    stack = [(root, True)]
    while stack:
        node, is_root = stack.pop()
        deleted = node.val in doomed
        if is_root and not deleted:
            forest.append(node)
        for child in (node.left, node.right):
            if child is not None:
                stack.append((child, deleted))
        if node.left is not None and node.left.val in doomed:
            node.left = None
        if node.right is not None and node.right.val in doomed:
            node.right = None
    return forest
```

## Complexity

- Time: O(n + d), where `d = len(to_delete)`: building the set, then O(1)
  work per node.
- Space: O(n + d) for the set and the stack.

## Pitfalls

- Checking membership in the `to_delete` list instead of a set makes the
  whole thing O(n * d).
- Forgetting to cut links to deleted children leaves deleted nodes inside the
  returned trees.
- Cutting a link before the child has been visited loses the child's own
  subtree, whose survivors still need to become roots.
- `to_delete` can mention values that aren't in the tree; ignore them.
