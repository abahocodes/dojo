# Approach: walk down one path

The ordering of a BST means the target can only be on one side of any node.
Start at the root and compare: go left when `val` is smaller, right when it is
larger, and stop when the values match or you reach a missing child. The node
you stop on is already the root of the subtree you need to return, so nothing
has to be copied.

```python
def search_bst(root: "TreeNode | None", val: int) -> "TreeNode | None":
    node = root
    while node is not None and node.val != val:
        node = node.left if val < node.val else node.right
    return node
```

## Complexity

- Time: `O(h)`, where `h` is the height of the tree. That is `O(log n)` for a
  balanced tree and `O(n)` for a degenerate one.
- Space: `O(1)`: the loop needs no recursion stack.

## Pitfalls

- Searching both subtrees works but throws away the BST property and costs
  `O(n)`.
- Return the node itself, not just its value or a fresh node with no
  children. The expected output is the whole subtree.
- A missing value must give an empty tree (`None`/`null`), not an error.
