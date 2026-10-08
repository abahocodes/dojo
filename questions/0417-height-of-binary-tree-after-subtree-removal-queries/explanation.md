# Approach: best and second-best per level

Let `depth[v]` be the root-to-`v` distance and `height[v]` the height of
`v`'s subtree. The longest root-to-node path through `v` has length
`depth[v] + height[v]`.

After deleting `v`'s subtree, any surviving path either passes through
another node `u` on the same level as `v` (length `depth[u] + height[u]`) or
stops above that level (length at most `depth[v] - 1`, and the path to
`v`'s parent always survives). So:

- answer = max of `depth[u] + height[u]` over other nodes `u` at `v`'s depth,
- or `depth[v] - 1` if `v` is the only node on its level.

Keeping the top two values per level answers each query in O(1).

```python
def tree_queries(root, queries):
    order, stack = [], [(root, 0)]
    while stack:
        node, d = stack.pop()
        order.append((node, d))
        if node.left:
            stack.append((node.left, d + 1))
        if node.right:
            stack.append((node.right, d + 1))
    n = len(order)
    depth, height = [0] * (n + 1), [0] * (n + 1)
    for node, d in reversed(order):  # children before parents
        depth[node.val] = d
        h = 0
        if node.left:
            h = height[node.left.val] + 1
        if node.right:
            h = max(h, height[node.right.val] + 1)
        height[node.val] = h
    levels = max(depth) + 1
    best1, best2, owner = [-1] * levels, [-1] * levels, [0] * levels
    for v in range(1, n + 1):
        d, reach = depth[v], depth[v] + height[v]
        if reach > best1[d]:
            best2[d] = best1[d]
            best1[d], owner[d] = reach, v
        elif reach > best2[d]:
            best2[d] = reach
    answer = []
    for q in queries:
        d = depth[q]
        if owner[d] != q:
            answer.append(best1[d])
        elif best2[d] >= 0:
            answer.append(best2[d])
        else:
            answer.append(d - 1)
    return answer
```

An equivalent approach does two DFS passes (left-first and right-first),
tracking the deepest node seen *before* entering each subtree.

## Complexity

- Time: O(n + q).
- Space: O(n) for depths, heights and the traversal order.

## Pitfalls

- Ties: if two nodes on a level share the best value, removing either one
  still leaves the other, so the answer is unchanged. Tracking the owner of
  `best1` plus a separate `best2` handles this automatically.
- A node alone on its level: the answer is `depth - 1` (its parent's level),
  not `-1` or `0`.
- With up to 10^5 nodes, the tree can be a 10^5-long chain; compute heights
  iteratively (reverse preorder) instead of with deep recursion.
