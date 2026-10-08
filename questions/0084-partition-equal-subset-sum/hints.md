# Hints

## Hint 1
If the total weight is odd, the answer is immediately `false`. Otherwise, the
question becomes: is there a group of boxes whose weights add up to exactly
half of the total?

## Hint 2
That's a 0/1 knapsack question. Keep track of which sums are reachable using
some subset of the boxes seen so far. Each new box `x` adds `s + x` for every
sum `s` that was already reachable.

## Hint 3
Use a boolean array `can[0..half]` with `can[0] = true`. For each box, sweep
`s` from `half` **down** to `x` and set `can[s] |= can[s - x]`. Going downwards
makes sure each box is used at most once.
