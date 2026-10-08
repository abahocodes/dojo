# Hints

## Hint 1
Summing each rectangle cell by cell costs up to `m * n` per query, which is too
slow for `10^4` queries on a `200 x 200` matrix. In one dimension, how do you
answer range-sum queries in O(1)?

## Hint 2
Build a table `pre[i][j]` holding the sum of the rectangle from `(0, 0)` to
`(i - 1, j - 1)`. Use an extra row and column of zeros so the edges need no
special cases. How can each entry be built from its neighbours?

## Hint 3
`pre[i+1][j+1] = matrix[i][j] + pre[i][j+1] + pre[i+1][j] - pre[i][j]`
(the overlap is added twice, so subtract it). A query is then
`pre[r2+1][c2+1] - pre[r1][c2+1] - pre[r2+1][c1] + pre[r1][c1]`.
