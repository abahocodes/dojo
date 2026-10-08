# Hints

## Hint 1
Counting the nodes first and then walking to index `floor(n / 2) - 1` works in
two passes. Can you find the node *before* the middle in a single pass?

## Hint 2
A slow pointer that moves one step while a fast pointer moves two ends up in
the middle when the fast pointer runs out of list. You want to stop one node
earlier than that.

## Hint 3
Handle `n == 1` by returning `None`. Otherwise start `slow = head` and
`fast = head.next.next`; while `fast` and `fast.next` exist, advance `slow` by
one and `fast` by two. Then `slow.next = slow.next.next`.
