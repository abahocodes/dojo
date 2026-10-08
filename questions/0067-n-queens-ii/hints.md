# Hints

## Hint 1
No two queens can share a row, and there are `n` queens on `n` rows, so every
row holds exactly one queen. Place them one row at a time.

## Hint 2
When placing the queen of row `r`, a column `c` is forbidden if an earlier
queen uses the same column, the same `r - c` diagonal, or the same `r + c`
anti-diagonal.

## Hint 3
Backtrack row by row, keeping three sets: used columns, used `r - c` values
and used `r + c` values. Count one solution each time you get past the last
row. For extra speed, store each set as a bitmask.
