# Hints

## Hint 1
Treat each point as a node and every pair of points as a possible edge
weighted by its Manhattan distance. What classic structure connects all nodes
with the least total weight?

## Hint 2
You want a minimum spanning tree. Kruskal's algorithm (sort all edges, add
them with union-find) works, but there are about n²/2 edges. Prim's algorithm
grows the tree one node at a time instead.

## Hint 3
Dense Prim without a heap: keep `best[v]`, the cheapest cable from the
current tree to point `v`. Repeat `n` times: pick the cheapest point not yet
in the tree, add its `best` to the total, then lower `best` for every
remaining point using its distance to the newly added point. That's O(n²)
with no edge list at all.
