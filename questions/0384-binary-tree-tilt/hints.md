# Hints

## Hint 1
Computing each node's subtree sums from scratch costs O(n) per node, O(n^2)
overall. Can one traversal produce every subtree sum?

## Hint 2
A node's subtree sum is `node.val + left_sum + right_sum`. If you process
children before their parent (post-order), both child sums are ready when you
reach the parent.

## Hint 3
In post-order, for each node read `left_sum` and `right_sum` (0 for missing
children), add `abs(left_sum - right_sum)` to a running total, then record
`node.val + left_sum + right_sum` as this node's sum for its parent to use.
