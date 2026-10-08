# Hints

## Hint 1
The water level of a square is decided by the lowest "wall" on the cheapest
escape route to the border, not just by its four neighbours. Think of the
border as the boundary of a pool that you shrink inward.

## Hint 2
The weakest point of the current boundary is its lowest square: water inside
can never stand higher than it on that side. Start with all border squares as
the boundary and always expand from the lowest one, using a min-heap.

## Hint 3
Pop the lowest boundary square (with its effective level). For each unvisited
neighbour, it traps `max(0, level - height)` water, and it joins the boundary
with level `max(level, height)`. Mark it visited when you push it, and sum
the trapped water.
