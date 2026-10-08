# Approach: post-order sums and counts

The average of a node's subtree depends only on the subtree's sum and size,
and both combine trivially from the children:

- `sum(node) = node.val + sum(left) + sum(right)`
- `count(node) = 1 + count(left) + count(right)`

So we process children before parents. A recursive post-order traversal does
that directly; to stay safe on a 1000-deep chain, we build a preorder list with
an explicit stack and walk it backwards, which also puts every child before its
parent.

```python
def average_of_subtree(root):
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    totals = {}  # node -> (sum, count) of its subtree
    count = 0
    for node in reversed(order):
        s, c = node.val, 1
        for child in (node.left, node.right):
            if child:
                cs, cc = totals[child]
                s += cs
                c += cc
        totals[node] = (s, c)
        if s // c == node.val:
            count += 1
    return count
```

## Complexity

- Time: O(n): each node is combined once.
- Space: O(n) for the order list and the per-node totals.

## Pitfalls

- Rounding: the average is rounded **down** (integer division). All values are
  non-negative, so truncating division in Java, C++ and Go agrees with floor.
- Comparing with a floating-point average: `29 / 6 = 4.83` is not `4`, but the
  rounded-down average is.
- Leaves always count, since a one-node average is the node's own value.
