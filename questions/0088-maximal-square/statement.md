A solar farm is planned on a rectangular plot, described by `matrix`, where
every cell is either `"1"` (flat ground, usable) or `"0"` (rock, unusable).
The panels must cover a **square** block of cells, aligned with the grid,
in which every cell is usable.

Return the **area** (number of cells) of the largest such square. If no cell
is usable, return `0`.

## Example 1

```
matrix = [
  ["1", "0", "1", "1", "1"],
  ["1", "1", "1", "1", "1"],
  ["0", "1", "1", "1", "1"],
  ["1", "0", "1", "1", "0"]
]
output = 9      # the 3 x 3 block in rows 0-2, columns 2-4
```

## Example 2

```
matrix = [
  ["0", "1"],
  ["1", "0"]
]
output = 1
```

## Constraints

- `1 <= rows, cols <= 200`
- Every cell is `"0"` or `"1"`.
