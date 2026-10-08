# Hints

## Hint 1
If `u` and `v` were at the same depth, you could move both up one step at a
time until they meet. What do you need to compute first, and how do you handle
different depths?

## Hint 2
Moving up one step at a time costs `O(depth)` per query, which is `O(n)` on a
path. Precompute, for every node, its ancestor `2^k` levels up for each `k`.
Then any jump of `d` levels is a handful of power-of-two jumps (the set bits of
`d`).

## Hint 3
Get depths with a BFS from the root (no recursion: the tree can be a path).
Build `up[0] = parent` (the root points to itself) and
`up[k][v] = up[k-1][up[k-1][v]]`. For a query, lift the deeper node to the
other's depth; if they are now equal you are done. Otherwise, for `k` from
high to low, jump both whenever `up[k][u] != up[k][v]`. The answer is
`up[0][u]`.
