# Hints

## Hint 1
A direct approach: from every node, walk every downward path and count the
ones that hit the target. That is O(n^2) in the worst case. Can you count all
paths that *end* at a node in one step?

## Hint 2
Let `prefix(v)` be the sum of the values from the root down to `v`. A downward
path from `u` to `v` sums to `prefix(v) - prefix(parent of u)`. This is the
"subarray sum equals k" trick, applied to a root-to-node path.

## Hint 3
During a DFS, keep a hash map counting the prefix sums on the current
root-to-node path (start with `{0: 1}`). At node `v`, add
`count[prefix(v) - target_sum]` to the answer, increment `count[prefix(v)]`,
explore the children, then decrement it again when you leave `v`.
