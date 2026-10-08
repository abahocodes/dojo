# Hints

## Hint 1
Because the list is sorted, equal values are always next to each other.

## Hint 2
You only ever need to compare a node with the node right after it.

## Hint 3
Walk with `cur`. While `cur.next` exists: if `cur.next.val == cur.val`, skip it
with `cur.next = cur.next.next` (and do not move `cur`); otherwise advance
`cur = cur.next`.
