# Hints

## Hint 1
Summing each block directly costs O(k^2) per cell. Can you get the sum of any
rectangle of the matrix in constant time?

## Hint 2
Build a 2-D prefix-sum table with an extra row and column of zeros:
`pre[i][j]` is the sum of rows `0..i-1` and columns `0..j-1`.

## Hint 3
For cell `(i, j)`, clamp the block's rows to `[max(0, i-k), min(m-1, i+k)]` and
its columns the same way, then use inclusion-exclusion on the table:
`pre[r2+1][c2+1] - pre[r1][c2+1] - pre[r2+1][c1] + pre[r1][c1]`.
