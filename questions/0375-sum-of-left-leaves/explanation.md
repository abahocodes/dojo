# Approach: traverse while remembering which side each node hangs on

A node is a left leaf when it has no children and its parent reached it through
the left link. Carry that flag along with every node in an explicit stack; the
root starts with the flag off.

```python
def sum_of_left_leaves(root):
    total = 0
    stack = [(root, False)]
    while stack:
        node, is_left = stack.pop()
        if node.left is None and node.right is None:
            if is_left:
                total += node.val
            continue
        if node.left:
            stack.append((node.left, True))
        if node.right:
            stack.append((node.right, False))
    return total
```

## Complexity

- Time: O(n): every node is visited once.
- Space: O(h) for the stack, where `h` is the tree height.

## Pitfalls

- Counting the root when the tree has a single node: it is nobody's left
  child, so the answer is `0`.
- Counting every left child, including ones that have children of their own.
- Counting leaves that are right children.
- Values can be negative, so the sum can be negative too.
