# Approach: in-order traversal must be strictly increasing

An in-order traversal (left subtree, node, right subtree) of a binary search
tree lists its values in sorted order. Conversely, if the in-order sequence is
strictly increasing, every node is larger than everything to its left and
smaller than everything to its right. So we traverse in order and compare each
value with the previous one, stopping at the first violation. The explicit
stack avoids recursion limits on deep trees.

```python
def is_valid_bst(root):
    stack = []
    prev = None
    node = root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        if prev is not None and node.val <= prev:
            return False
        prev = node.val
        node = node.right
    return True
```

**Alternative (bounds):** traverse with `(node, low, high)` triples. The root
gets `(-inf, inf)`; a left child gets `(low, node.val)` and a right child
`(node.val, high)`. Every node must satisfy `low < node.val < high`.

## Complexity

- Time: O(n): each node is pushed and popped once.
- Space: O(h) for the stack, O(n) for a skewed tree.

## Pitfalls

- Only comparing a node with its direct children (Example 2 passes that check).
- Using `<` instead of `<=` when comparing with the previous value: duplicates
  are not allowed.
- Using sentinel bounds like `-2^31` or `2^31 - 1`: those are legal node values.
  Use `None`/infinity, or the in-order approach, which needs no sentinel.
- Writing `if prev and ...`: a previous value of `0` is falsy in Python.
