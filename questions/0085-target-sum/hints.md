# Hints

## Hint 1
Trying all 2^n sign patterns works for tiny inputs but repeats a lot of work:
many different prefixes reach the same running total. Count how many ways
reach each running total instead of listing the ways.

## Hint 2
Split the numbers into the ones with `+` (sum `P`) and the ones with `-` (sum
`N`). Then `P - N = target` and `P + N = total`, so `P = (total + target) / 2`.

## Hint 3
So the answer is the number of subsets whose sum is `(total + target) / 2`
(zero if that isn't a non-negative integer). Count them with a 0/1 knapsack:
`ways[0] = 1`, and for each number `x` sweep `s` downwards doing
`ways[s] += ways[s - x]`.
