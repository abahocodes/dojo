# Hints

## Hint 1
A set of `n` items has `2^n` subsets. For each element, how many choices do
you make about whether it ends up in a subset?

## Hint 2
Each element is either in or out, independently. You can model that as a
recursion that branches twice per element, or as the `n` bits of a number
from `0` to `2^n - 1`.

## Hint 3
Backtracking: `backtrack(start)` records a copy of the current subset, then for
each `i >= start` it appends `nums[i]`, recurses with `i + 1`, and pops. Only
moving forward (`i + 1`) keeps a subset from being generated twice.
