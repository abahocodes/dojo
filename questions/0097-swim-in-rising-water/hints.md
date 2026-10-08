# Hints

## Hint 1
For a fixed path from start to finish, when can you swim it? Rephrase the
question in terms of the cells on the path.

## Hint 2
You can swim a path at time `t` exactly when every cell on it has height
`<= t`. So the answer is the smallest possible value of "the highest cell on
the path", over all paths. That's a minimax path problem.

## Hint 3
Run a Dijkstra-style search with a min-heap keyed by cell height. Always
expand the lowest reachable cell next, and track the highest height popped so
far. The moment you pop the bottom-right cell, that running maximum is the
answer. (Binary searching on `t` with a BFS check also works.)
