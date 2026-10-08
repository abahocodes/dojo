# Approach: DFS with pruning

Do a depth-first traversal with an explicit stack and let each node's value
decide which children are worth visiting:

- `node.val < low`: the node and its whole left subtree are too small, so only
  the right child is explored.
- `node.val > high`: the node and its whole right subtree are too large, so only
  the left child is explored.
- Otherwise the value counts, and both sides may hold more in-range values.

```python
def range_sum_bst(root: "TreeNode | None", low: int, high: int) -> int:
    total = 0
    stack = [root] if root else []
    while stack:
        node = stack.pop()
        if node.val < low:
            # everything on the left is even smaller
            if node.right:
                stack.append(node.right)
        elif node.val > high:
            # everything on the right is even larger
            if node.left:
                stack.append(node.left)
        else:
            total += node.val
            if node.left:
                stack.append(node.left)
            if node.right:
                stack.append(node.right)
    return total
```

## Complexity

- Time: `O(n)` in the worst case (when the range covers everything). With
  pruning, the work is about `O(h + m)`, where `m` is the number of in-range
  nodes and `h` the height.
- Space: `O(h)` for the stack in the usual case. The stack never holds more
  than `O(n)` nodes.

## Pitfalls

- Both ends are inclusive: a node equal to `low` or `high` counts.
- When a node is out of range, still explore its child on the side facing
  the range. A node below `low` can have a right subtree that is in range.
- A recursive solution can run very deep on a degenerate tree with `2 * 10^4`
  nodes. An explicit stack avoids that.
