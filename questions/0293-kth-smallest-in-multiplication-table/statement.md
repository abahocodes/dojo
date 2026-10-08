Picture an `m`-row, `n`-column multiplication table: the cell in row `i` and
column `j` (both counted from 1) holds `i * j`.

List every one of the `m * n` cells' values in non-decreasing order, keeping
repeated values as separate entries. Return the `k`-th value of that list
(counting from 1).

## Example 1

```
m = 3, n = 4, k = 7
output = 4

table:  1  2  3  4
        2  4  6  8
        3  6  9 12
sorted: 1 2 2 3 3 4 4 6 6 8 9 12   -> the 7th entry is 4
```

## Example 2

```
m = 2, n = 5, k = 10
output = 10   # k = m * n asks for the largest cell, 2 * 5
```

## Constraints

- `1 <= m, n <= 3 * 10^4`
- `1 <= k <= m * n`

Building the whole table is too slow and too large at these sizes.
