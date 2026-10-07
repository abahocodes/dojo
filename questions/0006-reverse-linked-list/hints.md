# Hints

## Hint 1
Copying the values into an array and rebuilding works, but try to do it by just
changing `next` pointers.

## Hint 2
Walk the list once. For each node, its `next` should end up pointing at the node
that came *before* it. What do you need to remember so you don't lose the rest
of the list?

## Hint 3
Keep `prev = None` and `cur = head`. Loop: save `nxt = cur.next`, set
`cur.next = prev`, then advance `prev = cur`, `cur = nxt`. When `cur` is `None`,
`prev` is the new head.
