A black-and-white bitmap is given as a list of rows `matrix`. Every row is a
string of the same length, and each character is either `"1"` (an inked cell)
or `"0"` (a blank cell).

Find the largest rectangle whose sides run along the grid lines and that
covers **only** inked cells. Return its area, i.e. the number of cells it
covers. If there are no inked cells at all, return `0`.

## Example 1

```
matrix = ["10100",
          "10111",
          "11111",
          "10010"]
output = 6    # rows 1-2, columns 2-4 are all "1"
```

## Example 2

```
matrix = ["0110",
          "1111",
          "0110"]
output = 6    # rows 0-2, columns 1-2 (a 3 x 2 block)
```

## Constraints

- `1 <= len(matrix) <= 200`
- `1 <= len(matrix[i]) <= 200`, and all rows have the same length
- every character is `'0'` or `'1'`
