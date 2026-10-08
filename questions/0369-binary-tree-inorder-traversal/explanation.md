# Approach: iterative inorder with an explicit stack

Inorder visits a node only after its whole left subtree. Walk left from the
current node, pushing every node on the way. When there is no further left
child, the node on top of the stack is the next one in inorder: pop it, record
it, then do the same for its right subtree.

```python
def inorder_traversal(root):
    result = []
    stack = []
    cur = root
    while cur or stack:
        while cur:
            stack.append(cur)
            cur = cur.left
        cur = stack.pop()
        result.append(cur.val)
        cur = cur.right
    return result
```

The recursive version is three lines:

```python
def walk(node):
    if node:
        walk(node.left)
        result.append(node.val)
        walk(node.right)
```

but its recursion depth equals the tree's height, which can be `10^4` here.

(Morris traversal achieves O(1) extra space by temporarily threading right
pointers, at the cost of modifying the tree while it runs.)

## Complexity

- Time: O(n) — each node is pushed and popped once.
- Space: O(h) for the stack, where `h` is the tree height (O(n) worst case).

## Pitfalls

- Recursing on a degenerate (chain-shaped) tree overflows the call stack in
  Python and can in other languages.
- Recording a node when it is pushed gives preorder, not inorder.
- The loop condition must check both `cur` and the stack; checking only the
  stack stops before visiting the right subtree of the last popped node.
