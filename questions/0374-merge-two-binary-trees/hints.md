# Hints

## Hint 1
Think about a single pair of nodes at the same position. If one of them is
missing, what can you return for that whole position without looking further?

## Hint 2
When both nodes exist, add the second value into the first and then solve the
same problem for the two left children and for the two right children.

## Hint 3
Merge into `root1`. Keep a stack of `(a, b)` pairs where `a` is a node of
`root1`. Pop a pair, skip it if `b` is missing, add `b.val` to `a.val`, then for
each side: if `a` has no child there, attach `b`'s child; otherwise push the
two children as a new pair.
