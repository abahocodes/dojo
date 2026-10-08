# Hints

## Hint 1
Instead of asking every node "is my grandparent even?", turn the question
around: what should an even-valued node contribute?

## Hint 2
An even-valued node contributes the values of its grandchildren: the children
of its left child and the children of its right child.

## Hint 3
Visit every node once (with an explicit stack or a queue). When a node's value
is even, add the values of up to four grandchildren, checking each pointer for
null before reading it.
