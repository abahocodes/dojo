# Approach: one post-order pass computing subtree sums

The tilt of a node only needs the sums of its two subtrees. A post-order
traversal produces every subtree sum bottom-up: when a node is handled, both
children's sums are already known. While computing them we add each node's
tilt to the total.

To stay iterative, collect nodes in pre-order (root, then children); reading
that list backwards visits every child before its parent.

```python
def find_tilt(root):
    if root is None:
        return 0
    order, stack = [], [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    subtree_sum = {}
    tilt = 0
    for node in reversed(order):
        left = subtree_sum.get(node.left, 0)
        right = subtree_sum.get(node.right, 0)
        tilt += abs(left - right)
        subtree_sum[node] = node.val + left + right
    return tilt
```

The recursive form is a helper that returns its subtree's sum and adds the
tilt to an outer variable as a side effect.

## Complexity

- Time: O(n): each node is processed once.
- Space: O(n) for the order list and the sum map.

## Pitfalls

- Using the children's *values* instead of the sums of their whole subtrees.
- Returning the root's tilt alone instead of the sum over all nodes.
- Recomputing subtree sums for every node, which is O(n^2) on a long chain.
- Forgetting the absolute value: sums can be negative.
