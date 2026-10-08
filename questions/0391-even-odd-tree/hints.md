# Hints

## Hint 1
Both rules talk about a whole level read left to right. Which traversal visits
the nodes in exactly that order?

## Hint 2
Do a breadth-first search one level at a time, keeping the level's index and
the previous value seen on that level.

## Hint 3
On an even level, fail if a value is even or is `<=` the previous one; on an
odd level, fail if a value is odd or is `>=` the previous one. Reset "previous"
at the start of every level. If every level passes, return `true`.
