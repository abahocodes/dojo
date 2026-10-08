# Hints

## Hint 1
Number the slots of each level as in an array-backed heap: if a node is at
position `p`, its children are at `2p` and `2p + 1`. The width of a level is
`last position - first position + 1`.

## Hint 2
A breadth-first search that carries `(node, position)` pairs gives you the
first and last positions of every level directly.

## Hint 3
On a deep, lopsided tree positions double every level and overflow even 64-bit
integers. Since only differences matter, subtract each level's first position
from all positions on that level before computing the children's positions.
