# Approach: row-by-row backtracking with attack sets

This is the search from N-Queens II, recording each placement instead of
counting it. Every row holds exactly one queen, so place queens row by row
and remember the column used in each row. A square `(r, c)` is under attack
if an earlier queen shares its column, its diagonal (same `r - c`) or its
anti-diagonal (same `r + c`). Keep those three in sets, so checking a square
is O(1), and undo them when backtracking.

When the last row is filled, the chosen columns describe the whole board:
row `r` is `c` dots, a `Q`, then `n - 1 - c` dots.

```python
def solve_n_queens(n):
    cols, diag, anti = set(), set(), set()
    queen_col = [0] * n
    boards = []

    def place(r):
        if r == n:
            boards.append(["." * c + "Q" + "." * (n - 1 - c) for c in queen_col])
            return
        for c in range(n):
            if c in cols or r - c in diag or r + c in anti:
                continue
            queen_col[r] = c
            cols.add(c); diag.add(r - c); anti.add(r + c)
            place(r + 1)
            cols.remove(c); diag.remove(r - c); anti.remove(r + c)

    place(0)
    return boards
```

**Faster checks:** replace the sets with boolean arrays (offset `r - c` by
`n - 1` so it is never negative) or with bitmasks as in N-Queens II. The
search is the same; only the constant factor changes.

## Complexity

- Time: O(n!) as an upper bound on the search (row `r` has at most `n - r`
  free columns), plus O(n²) to draw each of the `S` solutions found.
- Space: O(n) for the recursion and the attack sets, plus O(S · n²) for the
  output.

## Pitfalls

- Diagonals need two separate keys: `r - c` for one direction and `r + c` for
  the other. Checking only one lets queens attack each other.
- Copy the board when recording it. Appending a shared mutable grid and then
  undoing moves leaves every recorded board empty.
- Remove the queen's column and diagonals after the recursive call, or later
  branches see phantom queens.
- `n = 2` and `n = 3` have no solutions: return `[]`.
