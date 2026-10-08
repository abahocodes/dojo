# Hints

## Hint 1
Recomputing both sums from scratch for every index works, but costs O(n^2).
What do the left totals of two neighbouring indices have in common?

## Hint 2
The left total grows by exactly one element each time you move right. If you
also know the sum of the whole array, how do you get the right total?

## Hint 3
Compute `total = sum(nums)` once. Walk left to right keeping `left`; at index
`i` the right total is `total - left - nums[i]`. Record `|left - right|`, then
add `nums[i]` to `left`.
