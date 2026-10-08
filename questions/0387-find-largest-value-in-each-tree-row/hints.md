# Hints

## Hint 1
You need one answer per level. Which traversal naturally handles a tree one
level at a time?

## Hint 2
Breadth-first search with a queue, processing exactly the nodes of the current
level before moving on (snapshot the queue's size, or build a separate list
for the next level).

## Hint 3
For each level, start `best` at the value of its first node (not at `0`:
values can be negative), take the max over the level while collecting the
children, and append `best` to the result.
