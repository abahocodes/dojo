# Approach: in-order traversal, compare neighbours

An in-order traversal (left subtree, node, right subtree) of a BST produces the
values in increasing order. In a sorted sequence, the closest pair is
always two adjacent values: any other pair spans at least one value in between
and so has a larger gap. So walk the tree in order, keep the previous value,
and take the minimum of `current - previous`.

```python
def get_minimum_difference(root: "TreeNode") -> int:
    # Inorder lists the values in ascending order, so only neighbours matter.
    best = float("inf")
    prev = None
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        if prev is not None:
            best = min(best, node.val - prev)
        prev = node.val
        node = node.right
    return best
```

## Complexity

- Time: `O(n)`: every node is pushed and popped once.
- Space: `O(h)` for the stack, where `h` is the tree's height.

## Pitfalls

- The closest values are often not parent and child. In the second example,
  `35` is the right child of `20`, but its in-order neighbour `50` is the root.
  Only in-order neighbours are guaranteed to give the minimum.
- Do not compute a difference for the first visited node, since it has no
  predecessor. Start with "no previous value", not `0`.
- On a degenerate tree with `10^4` nodes, recursion can get deep. An
  explicit stack avoids that.
