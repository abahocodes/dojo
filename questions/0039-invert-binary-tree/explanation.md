# Approach: swap children at every node

The mirror of a tree is obtained by swapping the two children of every node.
The order in which you visit nodes does not matter, so a simple stack-based
traversal works and avoids recursion depth limits on very deep trees.

```python
def invert_tree(root):
    stack = [root]
    while stack:
        node = stack.pop()
        if node:
            node.left, node.right = node.right, node.left
            stack.append(node.left)
            stack.append(node.right)
    return root
```

The recursive version is a three-liner: swap
`root.left, root.right = invert_tree(root.right), invert_tree(root.left)` and
return `root`. It is fine for balanced trees, but a skewed tree with thousands
of nodes needs thousands of stack frames.

## Complexity

- Time: O(n): each node is visited once.
- Space: O(h) for the stack in a DFS (O(n) in the worst case), or O(w) for a
  BFS queue where `w` is the widest level.

## Pitfalls

- Swapping with two separate assignments (`node.left = node.right`, then
  `node.right = node.left`) loses a subtree; use a temporary or tuple swap.
- Forgetting to return the root.
- Deep recursion on skewed trees.
