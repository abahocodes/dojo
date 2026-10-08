# Hints

## Hint 1
Proportional pay means the whole group is paid one common rate `r` per unit
of quality. For everyone's minimum to be met, `r` must be at least
`wage[i] / quality[i]` for every hired worker. So the group's cost is
`(largest ratio in the group) * (sum of qualities in the group)`.

## Hint 2
Fix which worker has the largest ratio: call them the "captain". Every other
member must have a ratio no larger than the captain's, and among those
candidates you want the `k - 1` with the smallest qualities.

## Hint 3
Sort workers by ratio and scan in that order, so each worker in turn is the
captain and every earlier worker is eligible. Keep the `k` smallest
qualities seen so far in a max-heap (pop the largest when it grows past `k`)
along with their sum, and evaluate `sum * ratio` whenever the heap holds
exactly `k` workers.
