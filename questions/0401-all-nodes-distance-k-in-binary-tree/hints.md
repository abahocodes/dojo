# Hints

## Hint 1
Going down from the target is easy. The hard part is going *up*, because nodes
don't know their parents. How could you fix that?

## Hint 2
Once each node can reach its parent as well as its children, the tree is just
an undirected graph. What search finds every node at an exact distance from a
starting point?

## Hint 3
Traverse the tree once to build an adjacency list (or a parent map). Then run
a BFS from the target, level by level, with a visited set so you never step
back the way you came. After `k` levels, the current frontier is the answer.
Stop early if the frontier empties.
