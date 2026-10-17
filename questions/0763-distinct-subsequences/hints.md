# Hints

## Hint 1
Generating every subsequence of `s` is hopeless at length 1000. Think about
the last character of `s`: either it is used as the last character of `t`, or
it is not used at all.

## Hint 2
Let `dp[i][j]` be the number of ways to form the first `j` characters of `t`
from the first `i` characters of `s`. Then `dp[i][0] = 1` (the empty target
has exactly one way: pick nothing) and `dp[0][j] = 0` for `j > 0`.

## Hint 3
`dp[i][j] = dp[i-1][j]` (skip `s[i-1]`), plus `dp[i-1][j-1]` when
`s[i-1] == t[j-1]` (use it). The answer is `dp[len(s)][len(t)]`. Keep a single
row and update `j` from right to left to get O(len(t)) memory.
