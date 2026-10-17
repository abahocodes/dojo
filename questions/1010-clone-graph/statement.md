You are given one node of a connected, undirected graph (or `None` for an
empty graph). Each node has an integer value `val` and a list `neighbors` of
the nodes it shares an edge with; an edge between `a` and `b` appears in both
`a.neighbors` and `b.neighbors`.

Implement `clone_graph(node)`: build a **deep copy** of the whole graph and
return the copy of `node`. The copy must consist entirely of new nodes, one
per original node, with the same values and the same edges between the
copies. No node of the copy may be an original node, and the original graph
must be left unchanged. For `None`, return `None`.

The `Node` class is defined at the top of the starter code.

## How the tests describe a graph

A graph with `n` nodes has values `1` to `n` and is written as `adj_list`,
where `adj_list[i]` lists the values of the neighbors of the node with value
`i + 1`. The provided driver, `run_clone_graph`, builds the graph, calls your
`clone_graph` with the node of value `1` (or `None` when `adj_list` is
empty), explores everything reachable from what you return, and writes it
back in the same format with each neighbor list **sorted**, so neighbor order
in your copy doesn't matter. If the copy reaches an original node, has two
nodes with the same value, or the original graph was changed, the driver
returns `[[-1]]` instead. You don't need to change the driver.

## Example 1

```
adj_list = [[2, 4], [1, 3], [2, 4], [1, 3]]
output   = [[2, 4], [1, 3], [2, 4], [1, 3]]
```

A square: `1 - 2 - 3 - 4 - 1`. The copy is another square of new nodes.

## Example 2

```
adj_list = [[3, 2], [1], [1]]
output   = [[2, 3], [1], [1]]
```

Node `1` is joined to `2` and `3`. The output lists neighbors sorted.

## Example 3

```
adj_list = [[]]
output   = [[]]
```

A single node with no edges.

## Constraints

- The graph has between `0` and `100` nodes, with values `1` to `n`.
- The graph is connected and undirected, with no self-loops and no repeated
  edges.
