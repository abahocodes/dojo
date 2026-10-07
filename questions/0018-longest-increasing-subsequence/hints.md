# Hints

## Hint 1
A classic O(n²) DP defines `dp[i]` = length of the LIS ending at index `i`. For each `i`, scan all `j < i` and extend `dp[j]` when `nums[j] < nums[i]`. Can you think of a way to avoid the inner scan?

## Hint 2
Maintain a list `tails` where `tails[k]` is the smallest possible tail value of any increasing subsequence of length `k + 1` seen so far. This list is always sorted, which is the key invariant.

## Hint 3
For each element `x`, binary-search in `tails` for the leftmost position where `tails[pos] >= x`. If `pos` is past the end, append `x` (a new longest length). Otherwise, replace `tails[pos]` with `x` to keep tails as small as possible. The final length of `tails` is the answer.
