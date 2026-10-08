# Hints

## Hint 1
Comparing every pair of nodes is `O(n^2)`. If the values were in a sorted list,
which pairs could possibly give the minimum?

## Hint 2
In a sorted list, the closest pair is always two neighbours. An in-order
traversal of a BST visits the values in ascending order.

## Hint 3
Do an in-order traversal (an explicit stack works well) and remember the
previous value you visited. At each node, update the answer with
`node.val - prev`.
