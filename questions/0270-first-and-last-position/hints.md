# Hints

## Hint 1
Finding *some* index of `target` with an ordinary binary search and then
walking left and right is `O(n)` when the whole array equals `target`. Look
for each end of the block with its own binary search instead.

## Hint 2
The first index is the first position whose value is `>= target` (a "lower
bound"). The last index is one before the first position whose value is
`> target`.

## Hint 3
Write one helper `first_at_least(x)` that returns the first index with
`nums[i] >= x` (or `n`). Then `first = first_at_least(target)`; if
`first == n` or `nums[first] != target`, return `[-1, -1]`; otherwise
`last = first_at_least(target + 1) - 1`. Values are integers, so
"`> target`" is the same as "`>= target + 1`".
