# Approach: turn the tree into a graph, then BFS

A tree with parent links is an undirected graph, and the nodes at distance `k`
are exactly BFS level `k` from the target. Build an adjacency list keyed by
value (values are unique) with one traversal, then expand the frontier `k`
times, marking visited nodes so the search never walks back.

```python
def distance_k(root, target, k):
    adj = {root.val: []}
    stack = [root]
    while stack:
        node = stack.pop()
        for child in (node.left, node.right):
            if child is not None:
                adj[node.val].append(child.val)
                adj[child.val] = [node.val]
                stack.append(child)
    frontier = [target]
    seen = {target}
    for _ in range(k):
        nxt = []
        for v in frontier:
            for w in adj[v]:
                if w not in seen:
                    seen.add(w)
                    nxt.append(w)
        frontier = nxt
        if not frontier:
            break
    return frontier
```

**Alternative:** a DFS that returns the target's distance from each node.
Whenever an ancestor lies at distance `d` from the target, collect nodes at
depth `k - d - 1` from it on the *other* side. It needs no extra graph but
is more fiddly to get right.

## Complexity

- Time: O(n): each node and edge is handled a constant number of times.
- Space: O(n) for the adjacency list, visited set and frontier.

## Pitfalls

- Without a visited set, BFS bounces between a node and its parent and
  reports wrong nodes (for example the target itself when `k = 2`).
- `k = 0` must return just `[target]`.
- `k` can be larger than any distance in the tree; return `[]` instead of
  looping 1000 times over an empty frontier.
