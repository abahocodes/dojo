# Hints

## Hint 1
How many edges does a tree with `n` nodes have? What does that let you rule
out immediately?

## Hint 2
A tree on `n` nodes has exactly `n - 1` edges. With exactly `n - 1` edges,
"connected" and "has no cycle" become equivalent, so you only need to check
one of them.

## Hint 3
Return `false` unless `len(edges) == n - 1`. Then add the edges one by one to
a union-find. If an edge's two ends already share a root, it closes a cycle:
return `false`. If every edge merges two different sets, return `true`.
