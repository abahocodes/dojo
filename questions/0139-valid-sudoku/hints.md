# Hints

## Hint 1
Each filled cell belongs to exactly one row, one column and one box. A
duplicate in any of those 27 units makes the grid invalid.

## Hint 2
Keep a "seen" record per unit: 9 for rows, 9 for columns, 9 for boxes. Scan
the grid once and check all three records for each digit.

## Hint 3
The box of cell `(r, c)` is `(r / 3) * 3 + c / 3` with integer division.
Each record can be a 9-entry boolean array, or even a 9-bit mask.
