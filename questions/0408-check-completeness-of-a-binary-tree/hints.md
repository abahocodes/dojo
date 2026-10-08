# Hints

## Hint 1
Completeness is about the order of nodes level by level, left to right. Which
traversal visits positions in exactly that order?

## Hint 2
Run a breadth-first search, but enqueue the missing children too (as `null`).
In a complete tree, what does the sequence of dequeued items look like?

## Hint 3
All real nodes must come before the first `null`. Keep a flag that flips when
you dequeue the first `null`; if you dequeue a real node after that, return
`false`. If the queue empties without that happening, return `true`.
