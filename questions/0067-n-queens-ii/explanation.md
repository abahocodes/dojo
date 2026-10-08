# Approach: row-by-row backtracking with attack sets

Each row holds exactly one queen, so place queens row by row. A square
`(r, c)` is under attack if an earlier queen shares its column, its
diagonal (same `r - c`) or its anti-diagonal (same `r + c`). Keep those three
in sets, so checking a square is O(1), and undo them when backtracking.

```python
def total_n_queens(n):
    cols, diag, anti = set(), set(), set()

    def place(r):
        if r == n:
            return 1                     # every row has a queen
        count = 0
        for c in range(n):
            if c in cols or r - c in diag or r + c in anti:
                continue
            cols.add(c); diag.add(r - c); anti.add(r + c)
            count += place(r + 1)
            cols.remove(c); diag.remove(r - c); anti.remove(r + c)
        return count

    return place(0)
```

**Faster (bitmasks):** represent the three sets as integers and shift the
diagonal masks as you move down a row, so the free squares of a row are
`~(cols | diag | anti) & ((1 << n) - 1)`. Take the lowest set bit with
`free & -free` to iterate over them. This is the same search with much
smaller constants.

## Complexity

- Time: O(n!) as an upper bound: row `r` has at most `n - r` free columns.
  Pruning by diagonals makes the real search far smaller.
- Space: O(n) for the recursion and the three sets.

## Pitfalls

- Diagonals need two separate keys: `r - c` for one direction and `r + c` for
  the other. Checking only one lets queens attack each other.
- `r - c` can be negative. That's fine for a set, but an array needs an
  offset of `n - 1`.
- Remove the queen's column and diagonals after the recursive call, or later
  branches see phantom queens.
