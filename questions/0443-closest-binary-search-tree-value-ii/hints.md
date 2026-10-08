# Hints

## Hint 1
In sorted order, the k closest values form a contiguous window around the
position where `target` would be inserted. With the in-order sequence you
could grow that window one step at a time from the middle.

## Hint 2
You do not need the whole sorted sequence. Two iterators are enough: one that
walks values `<= target` downward (predecessors) and one that walks values
`> target` upward (successors). Each step takes whichever is closer.

## Hint 3
Build both iterators with stacks while walking from the root to `target`: push
a node onto the predecessor stack and go right if `node.val <= target`, else
push it onto the successor stack and go left. Taking a predecessor pops a node
and pushes its left child plus that child's chain of right children;
a successor is symmetric. Repeat k times: O(h + k) total.
