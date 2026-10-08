# Hints

## Hint 1
In each row the negative entries form a suffix. If you knew where that suffix
starts, the row's count would be one subtraction.

## Hint 2
You could binary search each row for its first negative entry, which gives
`O(m log n)`. But look at how that starting column changes from one row to
the next: since columns are sorted too, it can only move left as you go
down.

## Hint 3
Start at the bottom-left corner. If the entry is negative, everything to its
right in that row is negative too: add `n - col` and move up a row. Otherwise
move right a column. Stop when you leave the grid.
