# Hints

## Hint 1
Counting `ways[x] = sum(ways[x - c])` over all coins counts *orderings*
(`2+3` and `3+2` separately). You need a way to fix the order in which coins
are considered so each multiset is built exactly once.

## Hint 2
Decide denominations one at a time: first how many of `coins[0]` to use, then
how many of `coins[1]`, and so on. Let `ways[i][x]` be the number of ways to
make `x` using only the first `i` denominations.

## Hint 3
That collapses into a single array: `ways[0] = 1`, then for each coin `c` (outer
loop) sweep `x` **upwards** from `c` to `amount` doing `ways[x] += ways[x - c]`.
Coins outside, amounts inside.
