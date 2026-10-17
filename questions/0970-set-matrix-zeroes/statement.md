You are given a grid of integers `matrix` with `m` rows and `n` columns.
Wherever the grid holds a `0`, that zero "spreads" across its whole row and
its whole column: every cell in that row and in that column becomes `0`.

Only the zeros present in the original grid spread. Cells that become `0`
because of another zero do not spread any further.

Return the resulting grid. You may modify `matrix` itself and return it.

## Example 1

```
matrix = [[5, 1, 2],
          [3, 0, 4],
          [7, 8, 9]]
output = [[5, 0, 2],
          [0, 0, 0],
          [7, 0, 9]]
```

## Example 2

```
matrix = [[1, 0, 3, 4],
          [2, 6, 7, 0],
          [9, 9, 9, 9]]
output = [[0, 0, 0, 0],
          [0, 0, 0, 0],
          [9, 0, 9, 0]]
```

Both zeros spread: column 1 and row 0 from the first, column 3 and row 1 from
the second.

## Constraints

- `1 <= m, n <= 200`
- `-2^31 <= matrix[i][j] <= 2^31 - 1`

**Follow-up:** copying the grid uses O(m * n) extra memory, and remembering
which rows and columns to clear uses O(m + n). Can you do it with O(1) extra
memory?
