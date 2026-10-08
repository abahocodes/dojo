# Hints

## Hint 1
Recomputing both sums for every index is O(n²). What single number lets you
get the right-hand sum from the left-hand sum?

## Hint 2
If `total` is the sum of the whole array and `left` the sum before index `i`,
the sum after it is `total - left - nums[i]`.

## Hint 3
Compute `total` once, then scan left to right, keeping `left` up to date.
Return the first `i` where `left == total - left - nums[i]`; scanning from the
left guarantees it is the smallest.
