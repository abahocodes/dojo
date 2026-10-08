# Hints

## Hint 1
There are O(n^2) groups, too many to visit one by one. Instead, for every
wizard `i`, count the groups in which `i` is the weakest, and add up their
sums all at once.

## Hint 2
With a monotonic stack find, for each `i`, the nearest index to the left with
a strictly smaller value and the nearest index to the right with a smaller or
equal value. That makes every group's weakest wizard unique even with ties.

## Hint 3
The groups where `i` is the weakest are `[l..r]` with `left < l <= i <= r < right`.
With prefix sums `P` (`P[0] = 0`), each group sums to `P[r+1] - P[l]`. Summing
over all `l` and `r` needs the sums of ranges of `P`, which a second prefix sum
`PP` (prefix sums of `P`) gives in O(1):
`(i - left) * (PP[right+1] - PP[i+1]) - (right - i) * (PP[i+1] - PP[left+1])`.
