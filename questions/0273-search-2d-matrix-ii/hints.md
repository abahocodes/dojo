# Hints

## Hint 1
Binary searching every row works in `O(m log n)`, but you can do better by
picking a starting cell where one comparison always rules out a whole row or
a whole column.

## Hint 2
Look at the top-right corner. Everything to its left in that row is smaller,
and everything below it in that column is larger. The bottom-left corner has
the mirror property.

## Hint 3
Start at row `0`, column `n - 1`. If the cell equals `target`, you are done.
If it is larger than `target`, the whole column below is larger too, so move
left. If it is smaller, the whole row to the left is smaller too, so move
down. Stop when you leave the grid.
