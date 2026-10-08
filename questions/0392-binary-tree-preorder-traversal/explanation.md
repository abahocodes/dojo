# Approach: explicit stack

Preorder visits a node before its subtrees, and the left subtree before the
right. With a stack: pop a node, record it, and push its children so that the
left one comes off first, which means pushing the right child first.

```python
def preorder_traversal(root):
    result = []
    stack = [root] if root else []
    while stack:
        node = stack.pop()
        result.append(node.val)
        if node.right:
            stack.append(node.right)
        if node.left:
            stack.append(node.left)
    return result
```

**Recursive version:** `visit(node)` appends `node.val`, then calls
`visit(node.left)` and `visit(node.right)`. It is correct but uses the call
stack, which can overflow on a 10^4-node chain in some languages.

**Morris traversal** reaches O(1) extra space by temporarily threading each
node's in-order predecessor back to it, at the cost of more complex code.

## Complexity

- Time: O(n): every node is pushed and popped once.
- Space: O(h) for the stack, where `h` is the tree height (O(n) worst case).

## Pitfalls

- Pushing left before right: the stack then pops the right subtree first.
- Pushing `null` children and forgetting to skip them when popped.
- Returning `null`/`None` instead of `[]` for an empty tree.
