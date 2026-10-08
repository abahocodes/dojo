# Hints

## Hint 1
Counting every value in a hash map works. What does the BST ordering tell you
about where equal values appear in an in-order traversal?

## Hint 2
An in-order traversal visits values in non-decreasing order, so all copies of a
value come one after another. You only ever need the count of the current run.

## Hint 3
Track `prev`, the current run length `count`, the best count so far `best` and a
list `modes`. At each node, extend or restart the run. If `count > best`, set
`best = count` and `modes = [val]`. If `count == best`, append `val`.
