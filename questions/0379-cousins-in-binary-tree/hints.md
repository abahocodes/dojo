# Hints

## Hint 1
For each of the two values you need two facts: its depth and its parent.

## Hint 2
A breadth-first search handles depth for you: it finishes one level before
starting the next. Look at nodes from their parent, so the parent is the node
you are currently expanding.

## Hint 3
Process the tree level by level. While expanding a level, note the parent of
any child holding `x` or `y`. At the end of the level: if both were found,
return whether their parents differ; if only one was found, return `false`.
