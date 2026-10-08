# Hints

## Hint 1
Where would a search for `val` end if `val` is not in the tree?

## Hint 2
The search ends at a node whose child in the direction of `val` is missing.
That empty slot is exactly where the new leaf belongs.

## Hint 3
Handle the empty tree first (return a new node). Otherwise loop: pick `left` or
`right` by comparing with `val`; if that child is null, attach the new node and
return the original root, else descend into it.
