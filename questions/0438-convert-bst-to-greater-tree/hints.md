# Hints

## Hint 1
If you visited the nodes from the largest value to the smallest, what would
you need to remember to compute each new value?

## Hint 2
A normal in-order traversal (left, node, right) is ascending. Swapping the
order to right, node, left visits the values in descending order.

## Hint 3
Keep a running total. During the reverse in-order traversal add the node's
original value to the total, then overwrite the node's value with the total.
Use an explicit stack: the tree can be a chain 10^4 nodes deep.
