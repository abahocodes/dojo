# Hints

## Hint 1
Split the problem in two: a helper that decides whether two trees are exactly
identical, and a search over every node of `root` as a possible starting point.

## Hint 2
That is O(n * m) in the worst case. To do better, turn each tree into a string
that captures both values and shape. A pre-order walk that also writes a marker
for every missing child does this, and identical subtrees give identical
strings.

## Hint 3
Encode each node as `,value` and each missing child as `,#`. Then `sub_root`
is a subtree of `root` exactly when its encoding is a substring of `root`'s.
The leading comma stops `2` from matching the tail of `12`.
