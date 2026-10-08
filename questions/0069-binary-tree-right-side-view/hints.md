# Hints

## Hint 1
The answer has exactly one value per depth. Which traversal groups nodes by
depth?

## Hint 2
Process the tree level by level (BFS). Within a level, the node you can see
from the right is simply the last one when the level is listed left to right.

## Hint 3
Keep the current level as a list. Append the value of its last node to the
answer, then build the next level from the children (left before right). An
alternative is DFS that visits right children first and records the first node
it reaches at each new depth.
