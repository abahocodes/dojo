A robot stands on the top-left cell of a grid with `m` rows and `n` columns.
It wants to reach the bottom-right cell, and on every move it steps exactly one
cell **right** or one cell **down**; it can never leave the grid.

Return the number of **different routes** from the top-left to the
bottom-right cell.

## Example 1

```
m      = 3
n      = 4
output = 10     # any order of 2 downs and 3 rights
```

## Example 2

```
m      = 1
n      = 9
output = 1      # a single row: right, right, ... right
```

## Constraints

- `1 <= m, n`
- `m + n <= 58`, which keeps the answer below `2^53` (safe in a JavaScript number)
