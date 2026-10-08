# Hints

## Hint 1
You only care about one level: the last one. Which traversal hands you the
tree one whole level at a time?

## Hint 2
Run a breadth-first search level by level. Each time you finish a level, you
do not yet know whether it is the last one, so keep its sum around.

## Hint 3
Loop: compute the sum of the current level and build the list of its children.
If that list of children is empty, the current level is the deepest, so return
its sum; otherwise move on to the children.
