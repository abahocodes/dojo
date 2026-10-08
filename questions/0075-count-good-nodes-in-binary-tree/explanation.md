# Approach: DFS carrying the path maximum

A node is good exactly when its value is at least as large as every value
above it, so the only thing we need from the path is its maximum. Pass that
maximum down the tree: each child inherits `max(path_max, node.val)`. Start
the root with its own value, so the root is always counted.

```python
def good_nodes(root):
    count = 0
    stack = [(root, root.val)]
    while stack:
        node, best = stack.pop()
        if node.val >= best:
            count += 1
            best = node.val
        if node.left:
            stack.append((node.left, best))
        if node.right:
            stack.append((node.right, best))
    return count
```

Each stack entry stores the maximum for its own path, so the order in which
nodes are visited doesn't matter. BFS with a queue works just as well.

## Complexity

- Time: O(n): every node is visited once.
- Space: O(h) for a depth-first stack, at most O(n).

## Pitfalls

- Using `>` instead of `>=`: a node equal to the path maximum is still good
  (Example 2).
- Starting the maximum at `0` instead of the root's value miscounts trees with
  negative values.
- A single shared "maximum" variable that isn't restored when backtracking
  leaks one branch's maximum into its sibling.
- Recursion on a skewed tree with thousands of nodes can hit the recursion
  limit; an explicit stack avoids it.
