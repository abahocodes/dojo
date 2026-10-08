# Approach: DP on the bottom-right corner

Let `side[r][c]` be the side length of the largest all-`"1"` square whose
bottom-right corner is `(r, c)`. If the cell is `"0"`, it's `0`. Otherwise the
square can only grow as far as all three neighbouring squares allow:

```
side[r][c] = 1 + min(side[r-1][c], side[r][c-1], side[r-1][c-1])
```

The three overlapping squares (up, left, up-left) together cover everything a
square of side `k + 1` needs except the corner cell itself, and if any of them
is smaller than `k`, a gap exists. Cells outside the grid count as `0`.

Only the previous row is needed, so one array of length `cols + 1` suffices,
with `prev_diag` remembering the up-left value before it's overwritten.

```python
def maximal_square(matrix):
    cols = len(matrix[0])
    side = [0] * (cols + 1)
    best = 0
    for row in matrix:
        prev_diag = 0
        for c in range(1, cols + 1):
            above = side[c]
            if row[c - 1] == "1":
                side[c] = 1 + min(above, side[c - 1], prev_diag)
                best = max(best, side[c])
            else:
                side[c] = 0
            prev_diag = above
    return best * best
```

## Complexity

- Time: O(rows × cols).
- Space: O(cols).

## Pitfalls

- Return the **area**, not the side length.
- The cells are the strings `"0"` and `"1"`; comparing with the integer `1`
  never matches.
- Using only the up and left neighbours misses holes in the diagonal corner,
  e.g. `[["0", "1"], ["1", "1"]]` has no 2 × 2 square.
- In the one-row version, read the old `side[c]` before overwriting it; it is
  the next cell's diagonal.
