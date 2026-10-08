# Hints

## Hint 1
The infection spreads one edge per minute in every direction, so the answer is
the distance (in edges) from the start node to the node farthest from it.

## Hint 2
Distances that go *up* the tree are the problem: nodes don't know their
parents. One pass over the tree can record each node's parent, after which the
tree is just an undirected graph.

## Hint 3
With parent links in hand, run a breadth-first search from the start node over
`left`, `right` and `parent`, with a visited set, processing one layer per
minute. The number of layers minus one is the answer. Build the parent map
with an explicit stack: the tree can be a long path.
