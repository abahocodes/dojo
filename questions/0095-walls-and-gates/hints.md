# Hints

## Hint 1
Running a BFS from every empty room (or from every gate separately) works,
but it repeats a lot of work. Can one search handle all the gates at once?

## Hint 2
Put **every** gate into the BFS queue before you start. A breadth-first
search then reaches each room for the first time from whichever gate is
closest.

## Hint 3
Multi-source BFS: queue all cells with value `0`. Pop a cell, and for each
neighbour that still holds `2147483647`, set it to the current cell's value
plus one and queue it. A room that already has a smaller number is never
overwritten, because BFS reaches cells in order of distance.
