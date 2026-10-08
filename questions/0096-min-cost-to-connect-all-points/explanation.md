# Approach: Prim's algorithm on the complete graph

The cheapest network that connects every point is a **minimum spanning tree**
(MST) of the complete graph whose edge weights are Manhattan distances.

Every pair of points is an edge, so the graph is dense: about n²/2 edges.
For dense graphs the array version of Prim's algorithm fits best:

- `best[v]` is the cheapest known cable from the tree to point `v`
  (initially infinite, and `0` for the starting point).
- Repeat `n` times: choose the point outside the tree with the smallest
  `best`, add it to the tree and add `best` to the total. Then for every
  point still outside, update `best[v]` with the distance to the newly
  added point if that's cheaper.

```python
def min_cost_connect_points(points):
    n = len(points)
    INF = float("inf")
    best = [INF] * n
    in_tree = [False] * n
    best[0] = 0
    total = 0
    for _ in range(n):
        u = min((v for v in range(n) if not in_tree[v]), key=best.__getitem__)
        in_tree[u] = True
        total += best[u]
        ux, uy = points[u]
        for v in range(n):
            if not in_tree[v]:
                d = abs(points[v][0] - ux) + abs(points[v][1] - uy)
                if d < best[v]:
                    best[v] = d
    return total
```

**Alternatives:** Kruskal's algorithm (sort all O(n²) edges, then add them
with union-find, skipping edges inside one set) or Prim with a heap. Both are
O(n² log n) here because the edge count is already O(n²), and they use O(n²)
memory to hold the edges.

## Complexity

- Time: O(n²): `n` rounds, each scanning all points twice.
- Space: O(n).

## Pitfalls

- Building the full edge list for `n = 1000` makes about 500,000 edges.
  That works but is slow and memory-hungry compared to array-based Prim.
- A single point needs no cables: the answer is `0`.
- The distance is Manhattan (`|dx| + |dy|`), not Euclidean. No square roots,
  and everything stays an integer.
- With a heap-based Prim, a point can be pushed several times. Skip it when
  it's popped after already joining the tree.
