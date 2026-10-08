# Approach: post-order "best downward chain" with a global maximum

For every node compute `gain(node)`: the largest sum of a path that starts at
the node and goes straight down (through at most one child). Negative chains
are never worth extending, so

    gain(node) = node.val + max(0, gain(left), gain(right))

Any path has a highest node; at that node the path joins up to two downward
chains. So the best path topped at `node` is
`node.val + max(0, gain(left)) + max(0, gain(right))`, and the answer is the
maximum of that over all nodes. Children must be processed before parents. To
stay safe on deep trees, collect nodes in pre-order with a stack and handle them
in reverse.

```python
def max_path_sum(root):
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)

    gain = {}
    best = root.val
    for node in reversed(order):
        left = max(gain.get(node.left, 0), 0)
        right = max(gain.get(node.right, 0), 0)
        best = max(best, node.val + left + right)
        gain[node] = node.val + max(left, right)
    return best
```

The recursive version is the same idea: a helper returns `gain(node)` and
updates a `best` variable from the enclosing scope.

## Complexity

- Time: O(n): every node is processed once.
- Space: O(n) for the traversal order and the gain table (O(h) for a recursive
  version, plus its call stack).

## Pitfalls

- Starting `best` at 0: a tree of only negative values must return its largest
  (least negative) value, not 0.
- Returning the bent path (`val + left + right`) as the gain: a path that
  continues upward can only use one side.
- Not clamping child gains at 0, which forces bad subtrees into the path.
- Deep recursion on skewed trees.
