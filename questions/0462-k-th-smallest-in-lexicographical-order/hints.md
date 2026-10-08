# Hints

## Hint 1
Sorting `n` strings is far too slow for `n = 10^9`. Picture the numbers as a
tree: the root's children are `1..9`, and the children of `x` are `10x..10x+9`
(those that are `<= n`). In what order does a preorder traversal visit them?

## Hint 2
Preorder of that 10-ary tree is exactly dictionary order. Instead of visiting
nodes one at a time, can you count how many numbers lie in the subtree under a
prefix `p` without visiting them? Look at the range of `p`'s descendants level
by level: `[p, p]`, `[10p, 10p + 9]`, `[100p, 100p + 99]`, ...

## Hint 3
Start at `current = 1` with `k - 1` steps left. If the subtree of `current`
has `size <= steps`, skip it entirely (`steps -= size`, `current += 1`).
Otherwise descend (`steps -= 1`, `current *= 10`). Each level's contribution
is `min(n, last) - first + 1`; watch for overflow when multiplying by 10.
