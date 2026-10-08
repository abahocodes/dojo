# Approach: in-order relinking

An in-order traversal visits the nodes in increasing order, which is exactly
the order of the final chain. Keep a `tail` pointer, starting at a dummy node.
Each time a node is visited, hang it off `tail.right`, clear its left pointer
and make it the new tail.

Relinking during the traversal is safe:

- When a node is visited, its left subtree is finished, so clearing `left`
  loses nothing.
- When we set `tail.right = node`, the traversal has already moved into the
  old tail's right subtree. The old right pointer is no longer needed.

```python
def increasing_bst(root: "TreeNode") -> "TreeNode":
    dummy = TreeNode(0)
    tail = dummy
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        # its left subtree is already relinked, so the left pointer is free
        node.left = None
        tail.right = node
        tail = node
        node = node.right
    return dummy.right
```

**Alternative:** collect the values in order and build a brand-new chain. It is
simpler but allocates `n` new nodes.

## Complexity

- Time: `O(n)`.
- Space: `O(h)` for the traversal stack. The nodes are reused.

## Pitfalls

- Forgetting `node.left = None` leaves old left links in place. The result is
  no longer a chain and may even contain cycles.
- Without a dummy head, the first node needs special handling to become the
  new root.
- Return the new root (the smallest node), not the original root.
