# Hints

## Hint 1
Summing each range directly costs O(n) per query, which is O(n * q) overall:
too slow when both are 10^5.

## Hint 2
Precompute `prefix[i]`, the sum of the first `i` elements (`prefix[0] = 0`).
How do you get the sum of `nums[l..r]` from two prefix values?

## Hint 3
`sum(nums[l..r]) = prefix[r + 1] - prefix[l]`. Store the prefix sums in a
64-bit type, since they can reach 10^10.
