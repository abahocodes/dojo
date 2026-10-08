# Hints

## Hint 1
The `k`-th node from the front is easy. The `k`-th from the back is node number
`n - k + 1` from the front, so counting the nodes first gives a two-pass
solution.

## Hint 2
You can find both in a single pass with two pointers kept a fixed distance
apart.

## Hint 3
Walk `first` to the `k`-th node. Then start `second` at `head` and a runner at
`first`; advance both until the runner is on the last node. `second` is now the
`k`-th node from the end. Swap the two values.
