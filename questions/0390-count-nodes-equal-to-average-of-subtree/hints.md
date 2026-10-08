# Hints

## Hint 1
Recomputing each node's subtree from scratch is O(n^2). What could a node
learn from its children to avoid that?

## Hint 2
If every child reports the sum and the node count of its own subtree, a node
can compute its own sum and count in O(1). That means children must be handled
before their parent: a post-order traversal.

## Hint 3
For each node: `sum = val + left.sum + right.sum`, `count = 1 + left.count +
right.count`, and it counts when `sum // count == val`. To avoid deep
recursion, list the nodes in preorder with a stack and process that list in
reverse: every child then comes before its parent.
