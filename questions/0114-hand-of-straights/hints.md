# Hints

## Hint 1
If `len(hand)` is not a multiple of `group_size`, the answer is immediately
`false`. Otherwise, think about the smallest card in the hand.

## Hint 2
The smallest card can only be the **first** card of its run, since nothing
smaller is available to come before it. So its run is forced: it must be
`x, x+1, ..., x+group_size-1`.

## Hint 3
Count each value. Go through the distinct values in increasing order; if value
`x` still has `c` copies left, each must start a run, so subtract `c` from the
counts of `x .. x+group_size-1`. If any of those counts would go negative,
return `false`.
