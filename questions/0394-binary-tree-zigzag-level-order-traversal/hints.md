# Hints

## Hint 1
Forget the zigzag for a moment: how would you collect the values level by
level, each level left to right?

## Hint 2
A breadth-first search that processes one full level per iteration gives you
exactly that. The zigzag only changes how each finished level is written out.

## Hint 3
Keep a flag that flips after every level. Always enqueue children left then
right, and when the flag says "right to left", reverse the level's values (or
fill the level's array from the back) before appending it.
