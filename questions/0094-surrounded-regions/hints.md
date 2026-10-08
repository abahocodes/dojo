# Hints

## Hint 1
Instead of asking "which regions are surrounded?", ask the opposite: which
`"O"` cells are guaranteed to survive?

## Hint 2
An `"O"` survives exactly when it is connected to an `"O"` on the border.
Every other `"O"` gets captured.

## Hint 3
Start a flood fill (BFS or an explicit-stack DFS) from every border `"O"`
and mark everything it reaches as safe. Then build the answer: a cell is `"O"`
if it was marked safe, and `"X"` otherwise.
