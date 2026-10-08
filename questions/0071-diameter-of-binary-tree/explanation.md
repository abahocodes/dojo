# Approach: heights bottom-up, best turn point

Every path has a single highest node. For a fixed highest node, the longest
path through it descends as deep as possible into each subtree, so its length
is `height(left) + height(right)` edges (with heights counted in nodes and an
empty subtree having height `0`). The diameter is the maximum of that sum over
all nodes, and one post-order pass computes every height.

The reference solution produces a post-order without recursion. It lists nodes
in pre-order, reverses that list (so children come before their parent), and
stores heights in a dictionary.

```python
def diameter_of_binary_tree(root):
    order, stack = [], [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    height, best = {}, 0
    for node in reversed(order):
        left = height.get(node.left, 0)
        right = height.get(node.right, 0)
        best = max(best, left + right)
        height[node] = 1 + max(left, right)
    return best
```

The recursive version is shorter: a helper returns the height and updates
`best` in an enclosing scope. It is fine for balanced trees, but a chain of
thousands of nodes exceeds Python's default recursion limit.

## Complexity

- Time: O(n): each node is processed once.
- Space: O(n) for the traversal order and the heights.

## Pitfalls

- Returning `height(left) + height(right)` at the root only: the longest path
  may not pass through the root (Example 2).
- Mixing up nodes and edges: the answer counts edges, so a single node gives
  `0`, and a path through a node is `left + right`, not `left + right + 1`.
- Recomputing heights from scratch at every node is O(n^2) on a skewed tree.
