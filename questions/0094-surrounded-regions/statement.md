A rectangular `board` holds the strings `"X"` and `"O"`. Group the `"O"`
cells into **regions**: two `"O"` cells are in the same region when you can
step from one to the other through `"O"` cells, moving up, down, left or
right.

A region is **captured** when none of its cells lies on the outer border of
the board (first or last row, first or last column), so it is completely
fenced in by `"X"`. Every cell of a captured region turns into `"X"`. Regions
that touch the border stay as they are.

Return the board after all captures, as a new board of the same size.

## Example 1

```
board = [
  ["X", "X", "X", "X"],
  ["X", "O", "O", "X"],
  ["X", "X", "O", "X"],
  ["X", "O", "X", "X"]
]
output = [
  ["X", "X", "X", "X"],
  ["X", "X", "X", "X"],
  ["X", "X", "X", "X"],
  ["X", "O", "X", "X"]
]
# the middle region is captured; the "O" on the bottom row touches the border
```

## Example 2

```
board = [
  ["X", "X", "X", "X", "X"],
  ["X", "O", "X", "O", "X"],
  ["X", "X", "X", "X", "X"],
  ["X", "O", "O", "O", "X"],
  ["X", "X", "X", "X", "O"]
]
output = [
  ["X", "X", "X", "X", "X"],
  ["X", "X", "X", "X", "X"],
  ["X", "X", "X", "X", "X"],
  ["X", "X", "X", "X", "X"],
  ["X", "X", "X", "X", "O"]
]
# the corner "O" only touches the row-3 region diagonally, which doesn't count
```

## Constraints

- `1 <= rows, cols <= 200`
- Every cell is `"X"` or `"O"`.
