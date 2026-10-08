# Hints

## Hint 1
You could really delete leaves round after round, but that rescans the tree
each time. Can you tell, without simulating, in which round a given node will
be removed?

## Hint 2
A leaf goes in round 0. A node goes one round after the *last* of its children
goes. That number is the node's height: the length of the longest path from it
down to a leaf.

## Hint 3
Compute heights with a post-order DFS (`height(None) = -1`,
`height(node) = 1 + max(height(left), height(right))`) and append each value to
`result[height]`, creating that group when `height == len(result)`. Post-order
visits left subtrees first, so each group comes out left to right.
