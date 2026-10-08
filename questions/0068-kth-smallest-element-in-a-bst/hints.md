# Hints

## Hint 1
Which traversal of a binary search tree visits the values in increasing order?

## Hint 2
An in-order traversal (left subtree, node, right subtree) yields the sorted
sequence. You don't need the whole sequence: stop as soon as you have visited
`k` nodes.

## Hint 3
Do the in-order walk with an explicit stack: push nodes while going left, pop
one, count it, then move to its right child. When the count reaches `k`, the
popped node is the answer.
