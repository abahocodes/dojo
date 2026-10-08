# Hints

## Hint 1
Which O(n log n) sort does not need random access? Linked lists are cheap to
split and cheap to merge.

## Hint 2
Merge sort: split the list into two halves, sort each, then merge two sorted
lists — the merge needs no extra nodes, just pointer rewiring.

## Hint 3
Find the midpoint with slow/fast pointers (start `fast` at `head.next` so a
two-node list splits into one and one), cut with `slow.next = None`, recurse on
both halves and merge with a dummy head node.
