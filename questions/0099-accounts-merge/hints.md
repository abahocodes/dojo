# Hints

## Hint 1
Comparing every pair of accounts for shared emails is slow, and it misses
chains (A shares with B, B shares with C). What graph is hiding here?

## Hint 2
Accounts are nodes; a shared email is an edge. Each person is a connected
component. You don't need to compare accounts pairwise: remember, for each
email, the first account that listed it.

## Hint 3
Union-find over account indices. Walk every email: if it's been seen before,
union the current account with the account that first listed it; otherwise
record it. Then group emails by the root of their owner, sort each group,
prepend the root account's name, and sort the merged accounts by
`(name, first email)`.
