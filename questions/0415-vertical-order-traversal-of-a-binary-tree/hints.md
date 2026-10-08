# Hints

## Hint 1
First work out each node's `(row, col)` with any traversal, passing the
coordinates down from parent to child.

## Hint 2
Once you have a `(col, row, value)` triple for every node, the required
output order is just a sort. Which key goes first?

## Hint 3
Sort the triples by `(col, row, value)` and then sweep them, starting a new
output list whenever the column changes.
