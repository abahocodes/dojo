# Hints

## Hint 1
If some capacity works, every larger capacity works too. So the answers form
a threshold: "too small, too small, ..., works, works". Can you test a single
capacity quickly?

## Hint 2
For a fixed capacity, load greedily: keep adding the next package to today's
load until it would overflow, then start a new day. Greedy uses the fewest
days possible for that capacity. The capacity is at least `max(weights)` and
at most `sum(weights)`.

## Hint 3
Binary search the capacity in `[max(weights), sum(weights)]`: if the greedy
day count is `<= days`, the answer is this capacity or smaller (`hi = mid`),
otherwise it is larger (`lo = mid + 1`).
