# Approach: reverse in-order traversal with a running sum

Visiting a BST right subtree first, then the node, then the left subtree lists
the values from largest to smallest. If we keep the sum of everything visited
so far, then when we reach a node, that sum is exactly the total of all values
greater than it. Add the node's own value and store the result in the node.

An explicit stack replaces recursion so that a 10^4-deep chain is safe.

```python
def convert_bst(root):
    running = 0
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.right
        node = stack.pop()
        running += node.val
        node.val = running
        node = node.left
    return root
```

The largest possible total is 10^4 values of at most 10^4 each, 10^8, which
fits in a 32-bit int.

## Complexity

- Time: O(n): each node is pushed and popped once.
- Space: O(h) for the stack, where h is the tree height (up to n for a chain).

## Pitfalls

- Using normal in-order order (left first) accumulates the *smaller* values.
- Overwrite the value only after adding the original to the running sum, or
  you will add the already-converted value.
- Return the original root (the shape does not change); an empty tree returns
  null.
