# Hints

## Hint 1
Copying a node is easy; the trouble is that graphs have cycles. If you copy
neighbors recursively without remembering what you've already copied, you go
around a cycle forever, or create several copies of the same node.

## Hint 2
Keep a hash map from each original node to its copy. Before creating a copy,
check the map: if the node was already copied, reuse that copy. The map is
both your "visited" set and the way to find the copy to link to.

## Hint 3
Traverse with DFS or BFS from the given node. When you first meet a node,
create its copy, store it in the map and schedule the node for processing.
When you process a node, append `copies[neighbor]` to
`copies[node].neighbors` for every neighbor. Return `copies[start]`.
