# Hints

## Hint 1
The first value must stay first: it is the root. After that, the values
smaller than the root build the left subtree and the larger ones build the
right subtree. Does the relative order *between* a left value and a right
value matter?

## Hint 2
It does not. Only the order within the left group and within the right group
matters. If the left subtree has `L` nodes and the right `R`, the two groups
can be interleaved in `C(L + R, L)` ways, and each group must itself be an
ordering that builds its own subtree.

## Hint 3
So `ways(node) = C(L + R, L) * ways(left) * ways(right)`, with `ways(empty) = 1`,
and the answer is `ways(root) - 1`. Precompute factorials and inverse
factorials modulo `10^9 + 7` for the binomials. A sorted input makes a chain
1000 deep, so build the tree with insertion loops and evaluate it bottom-up:
processing `nums` in reverse handles every child before its parent.
