# Approach: edge count + union-find

A graph on `n` nodes is a tree when it is connected and acyclic. Any two of
these three facts imply the third:

1. it is connected,
2. it has no cycle,
3. it has exactly `n - 1` edges.

So first check the edge count. If it isn't `n - 1`, the answer is `false`.
Otherwise, it's enough to check for cycles. Union-find spots a cycle the
moment an edge joins two nodes that are already in the same set.

```python
def valid_tree(n, edges):
    if len(edges) != n - 1:
        return False
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return False      # this edge closes a cycle
        parent[ra] = rb
    return True
```

With `n - 1` edges and no cycle, the `n - 1` successful merges reduce `n`
sets to one, so the graph is connected.

**Alternative:** check the edge count, then run BFS/DFS from node `0` and
check that it reaches all `n` nodes.

## Complexity

- Time: O(n · α(n)), effectively O(n), since after the count check there are
  only `n - 1` edges.
- Space: O(n).

## Pitfalls

- Checking only for cycles misses a forest: `n = 4, edges = [[0, 1], [2, 3]]`
  has no cycle but isn't connected.
- Checking only connectivity misses extra edges that form a cycle.
- `n = 1` with no edges is a valid tree (a single node).
- In a DFS-based cycle check on an undirected graph, the edge back to the node
  you just came from is not a cycle. Skip the parent.
