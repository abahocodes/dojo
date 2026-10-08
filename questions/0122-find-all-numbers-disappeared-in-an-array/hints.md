# Hints

## Hint 1
A boolean array of size `n + 1` marking which values you have seen solves it
in O(n) time.

## Hint 2
To avoid that extra array, reuse `nums` itself: every value `v` points to a
valid index, `v - 1`.

## Hint 3
For each value `v` (use its absolute value), make `nums[v - 1]` negative. In a
final pass, every index `i` that still holds a positive number means `i + 1`
never appeared.
