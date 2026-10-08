# Hints

## Hint 1
Checking every top-left corner and every size is far too slow. Instead, ask
for each cell: what is the side of the largest all-`"1"` square whose
**bottom-right** corner is this cell?

## Hint 2
A square of side `k` ending at `(r, c)` needs squares of side `k - 1` ending
at the cell above, the cell to the left, and the cell diagonally up-left.

## Hint 3
So `side[r][c] = 1 + min(side[r-1][c], side[r][c-1], side[r-1][c-1])` when the
cell is `"1"`, and `0` otherwise. Track the largest side and return its square.
