# Hints

## Hint 1
Fix a share size `x`. How many children can you serve with shares of exactly
`x` candies? Each pile contributes independently.

## Hint 2
Pile `i` yields `candies[i] // x` shares. If size `x` works, every smaller size
works too, so feasibility is monotone in `x`.

## Hint 3
Binary search the largest `x` in `[1, max(candies)]` whose share count
`sum(c // x)` is at least `k`. The count can exceed 32 bits, so keep it in a
64-bit integer (or stop summing once it reaches `k`).
