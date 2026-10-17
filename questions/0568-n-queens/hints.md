# Hints

## Hint 1
No two queens can share a row, and there are `n` queens on `n` rows, so every
row holds exactly one queen. Place them one row at a time, and remember the
column chosen in each row so you can draw the board at the end.

## Hint 2
When placing the queen of row `r`, a column `c` is forbidden if an earlier
queen uses the same column, the same `r - c` diagonal, or the same `r + c`
anti-diagonal.

## Hint 3
Backtrack row by row, keeping three sets: used columns, used `r - c` values
and used `r + c` values. When you get past the last row, turn the chosen
columns into strings (`'.' * c + 'Q' + '.' * (n - 1 - c)` for each row) and
record the board. Undo the three sets after each recursive call.
