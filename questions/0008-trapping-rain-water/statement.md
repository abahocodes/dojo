A row of columns stands side by side, each one unit wide; `height[i]` is the
height of column `i`. After a heavy rain, water settles wherever it is held in
by taller columns on both sides. Water spills off both ends of the row.

Return the total number of unit squares of water held between the columns.

## Example 1

```
height = [3, 0, 1, 0, 4, 1, 2]
output = 9
         # 3 + 2 + 3 above columns 1..3, plus 1 above column 5
```

## Example 2

```
height = [2, 5, 1, 1, 3]
output = 4         # 2 + 2 above the two 1s
```

## Constraints

- `1 <= len(height) <= 2 * 10^4`
- `0 <= height[i] <= 10^5`
