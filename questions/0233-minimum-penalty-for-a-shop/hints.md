# Hints

## Hint 1
Computing the penalty of each hour from scratch is O(n) per hour, O(n^2) in
total. How does the penalty change when the closing hour moves from `j` to
`j + 1`?

## Hint 2
Moving the closing time past hour `j` turns hour `j` from closed to open. If
`customers[j] == 'Y'` the penalty drops by one; if it is `'N'` it rises by
one. Nothing else changes.

## Hint 3
You do not even need the starting penalty: track the change relative to
`j = 0`, starting at `0`, and remember the first `j` at which that running
value reaches a new strict minimum.
