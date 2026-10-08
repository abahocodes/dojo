# Hints

## Hint 1
Rot spreads outward one step per minute from *every* rotten orange at once.
What search explores a grid in rings of increasing distance?

## Hint 2
Run a breadth-first search that starts from all rotten oranges together
(a multi-source BFS). Each BFS level is one minute.

## Hint 3
Count the fresh oranges first. Put every rotten cell in the starting frontier.
For each minute, rot every fresh neighbour of the frontier, decrement the
fresh count, and make those cells the next frontier. Stop when the frontier is
empty or nothing fresh is left; return the minutes if the count reached `0`,
otherwise `-1`.
