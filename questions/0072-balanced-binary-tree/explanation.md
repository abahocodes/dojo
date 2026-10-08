# Approach: one bottom-up pass over the heights

A node's height depends only on its children's heights, so a post-order pass
computes every height in O(n) total. While doing so, compare the two child
heights at each node and stop as soon as they differ by more than one.

To avoid recursion on very deep trees, the reference solution lists the nodes
in pre-order and walks that list backwards, which always handles children
before their parent:

```python
def is_balanced(root):
    if root is None:
        return True
    order, stack = [], [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    height = {}
    for node in reversed(order):
        left = height.get(node.left, 0)
        right = height.get(node.right, 0)
        if abs(left - right) > 1:
            return False
        height[node] = 1 + max(left, right)
    return True
```

The classic recursive form returns `-1` for "unbalanced" and propagates it
upward. It is equally fast, but limited by the recursion depth.

## Complexity

- Time: O(n): each node is visited once.
- Space: O(n) for the order list and the height table (O(h) for the recursive
  version's stack).

## Pitfalls

- Comparing heights only at the root: a tree whose root looks fine can still be
  unbalanced deeper down.
- Confusing "balanced" with "complete" or "perfect": a balanced tree may have
  missing nodes at several levels.
- Calling a separate `height()` function at every node costs O(n^2).
- An empty tree is balanced.
