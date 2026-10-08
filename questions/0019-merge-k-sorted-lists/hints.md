# Hints

## Hint 1
You already know how to merge two sorted linked lists in O(n + m). Merging k lists one at a time costs O(N · k) where N is the total node count. Can you avoid the linear-in-k factor?

## Hint 2
Instead of merging sequentially, keep a min-heap of the current head of every non-empty list. At each step, extract the smallest head, append it to the result, and push that list's next node back into the heap.

## Hint 3
Initialise the heap with the head of each non-empty list. Pop the minimum, link it to the result tail, and if it has a `next` node, push that. Repeat until the heap is empty. Each of the N nodes is pushed and popped exactly once, giving O(N log k) time.
