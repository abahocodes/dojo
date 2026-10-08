# Hints

## Hint 1
The shortest path goes up from the start to the lowest common ancestor of the
two nodes, then down to the destination. How many `'U'` steps is that, and what
are the downward steps?

## Hint 2
Write down the path from the root to each node as a string of `'L'`/`'R'`
moves. The two strings agree on a common prefix: that prefix is the walk from
the root to the lowest common ancestor.

## Hint 3
Strip the common prefix from both strings. Every remaining move in the start's
path becomes a `'U'`, and the destination's remaining moves are appended
unchanged: `'U' * len(rest_of_start) + rest_of_dest`. Build the root-to-node
paths iteratively (record each node's parent and the move into it) so deep
trees don't overflow the stack.
