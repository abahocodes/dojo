# Hints

## Hint 1
Simulating water from every cell separately repeats a lot of work. Can you
answer "which cells drain into the Pacific?" for all cells at once?

## Hint 2
Reverse the flow. Start from the cells that touch an ocean and walk *uphill*:
from a cell you may step to a neighbour whose height is greater than or equal
to the current one. Every cell you reach can drain into that ocean.

## Hint 3
Run that reverse search twice: once seeded with the whole top row and left
column (Pacific), once with the bottom row and right column (Atlantic). Each
produces a boolean grid. The answer is every cell marked in both.
