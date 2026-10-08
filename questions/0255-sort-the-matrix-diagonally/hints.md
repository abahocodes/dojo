# Hints

## Hint 1
Every cell `(i, j)` on the same diagonal has the same value of `i - j`.

## Hint 2
There are `m + n - 1` diagonals. Start one walk at each cell of the left
column and one at each cell of the top row (skipping `(0, 0)` the second
time).

## Hint 3
For each diagonal: copy its values out, sort them, and write them back in the
same walk order. Since values are at most 100, a 101-slot counting sort per
diagonal also works.
