# Hints

## Hint 1
Only three nodes matter: the node just before the removed stretch, the node
just after it, and the last node of `list2`.

## Hint 2
Walk `list1` from the head to find the node at position `a - 1` and the node at
position `b + 1`. Since `a >= 1` and `b + 1` is inside the list, both exist.

## Hint 3
Set `before.next = list2`, walk `list2` to its tail, then set
`tail.next = after`. Return `list1` (its head is never removed).
