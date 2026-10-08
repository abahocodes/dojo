# Hints

## Hint 1
A breadth-first search processes the tree level by level. Which node of the
last level do you want, and when does BFS reach it?

## Hint 2
With a normal left-to-right BFS you can remember the first node of each level;
the one remembered last belongs to the deepest level.

## Hint 3
Even simpler: run the BFS right-to-left (enqueue the right child before the
left one). Then the very last node dequeued is the leftmost node of the
deepest level; return its value.
