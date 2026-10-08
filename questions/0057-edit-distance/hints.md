# Hints

## Hint 1
Focus on the last characters of the two strings. If they're equal, do they
ever need to be touched?

## Hint 2
Let `D(i, j)` be the cost of turning `word1[:i]` into `word2[:j]`. If the last
characters differ, the final edit is a delete, an insert or a replace. Each one
leaves a smaller prefix pair to solve.

## Hint 3
`D(i, 0) = i`, `D(0, j) = j`. If `word1[i-1] == word2[j-1]` then
`D(i, j) = D(i-1, j-1)`; otherwise `1 + min(D(i-1, j), D(i, j-1), D(i-1, j-1))`
for delete, insert and replace. Fill the table row by row.
