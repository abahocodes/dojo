# Approach: count seam positions

A vertical line at horizontal position `x` avoids a brick in row `r` exactly
when `x` is one of that row's seams, that is, a prefix sum of the row's
widths strictly between `0` and the total width. So the best line sits at the
position shared by the most rows' seams.

Scan every row, accumulate the widths of all bricks but the last, and count
how many rows have a seam at each prefix sum. If the most popular position is
shared by `best` rows, the line crosses `len(wall) - best` bricks. With no
interior seams at all, `best` is `0` and every row is crossed.

```python
def least_bricks(wall):
    seams = {}
    best = 0
    for row in wall:
        pos = 0
        for width in row[:-1]:
            pos += width
            seams[pos] = seams.get(pos, 0) + 1
            best = max(best, seams[pos])
    return len(wall) - best
```

## Complexity

- Time: O(B), where B is the total number of bricks.
- Space: O(B) for the map of seam positions.

## Pitfalls

- Counting the final prefix sum of each row. It equals the total width, the
  outer edge, where the line is not allowed; every row would "match" there.
- Simulating every integer position: the width can be about two billion.
- Forgetting the case with no interior seams (every row is one brick): the
  answer is the number of rows.
