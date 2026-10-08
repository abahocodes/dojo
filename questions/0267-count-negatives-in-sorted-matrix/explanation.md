# Approach: staircase walk

In each row the negatives are a suffix, and because columns are sorted
non-increasingly, the first negative column of a row is never to the left of
the first negative column in the row below it. So the boundary between
non-negative and negative entries is a staircase.

Walk along it from the bottom-left corner `(m - 1, 0)`:

- if `grid[row][col] < 0`, the whole rest of this row (`n - col` entries) is
  negative; count them and move up (`row -= 1`);
- otherwise this entry and everything above it in the column are
  non-negative, so move right (`col += 1`).

```python
def count_negatives(grid):
    m, n = len(grid), len(grid[0])
    row, col = m - 1, 0
    count = 0
    while row >= 0 and col < n:
        if grid[row][col] < 0:
            count += n - col
            row -= 1
        else:
            col += 1
    return count
```

Every step moves up or right, so the walk takes at most `m + n` steps.

## Complexity

- Time: `O(m + n)`.
- Space: `O(1)`.

## Pitfalls

- Counting zeros as negative. The test is `< 0`, not `<= 0`.
- Starting at the top-left or bottom-right corner: from there both directions
  look the same and you cannot discard a row or column per step.
- Assuming the matrix is square or that each row has at least one negative.
