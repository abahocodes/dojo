Lay a binary tree out on a grid. The root sits at row `0`, column `0`. If a node
sits at `(row, col)`, its left child sits at `(row + 1, col - 1)` and its right
child at `(row + 1, col + 1)`.

Return the values column by column, from the leftmost column to the
rightmost. Within one column, list values from the top row down. When two or
more nodes share the same row **and** column, list them in increasing order
of value.

## Example 1

```
root   = [3, 9, 20, null, null, 15, 7]
output = [[9], [3, 15], [20], [7]]
```

## Example 2

```
root   = [1, 2, 3, 4, 6, 5, 7]
output = [[4], [2], [1, 5, 6], [3], [7]]
```

`6` (right child of `2`) and `5` (left child of `3`) both land at row `2`,
column `0`, so the smaller value `5` comes first.

## Constraints

- The tree has between `1` and `1000` nodes.
- `0 <= node.val <= 1000`
- Values may repeat.
