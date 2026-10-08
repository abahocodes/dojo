# Hints

## Hint 1
Sorting makes duplicates adjacent, which makes it easy to skip groups you
have already reported.

## Hint 2
Fix the first two values with two nested loops. The remaining two must sum to
`target - nums[i] - nums[j]` and lie to the right of `j`: that is the classic
sorted two-pointer pair search.

## Hint 3
After recording a match, move both pointers past every copy of their values.
Likewise, skip `i` (and `j`) when its value equals the previous one at the
same loop level. Compute sums in 64-bit integers.
