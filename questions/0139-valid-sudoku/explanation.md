# Approach: one pass with 27 seen-sets

Each filled cell must be the only one with its digit in its row, its column
and its box. Number the boxes 0..8 with `box = (r // 3) * 3 + c // 3`, and
keep a 9-bit mask for each row, column and box. For every filled cell, if its
digit's bit is already set in any of the three masks, a digit repeats;
otherwise set the bit in all three.

```python
def is_valid_sudoku(board):
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9
    for r in range(9):
        for c in range(9):
            ch = board[r][c]
            if ch == ".":
                continue
            bit = 1 << (ord(ch) - ord("1"))
            b = (r // 3) * 3 + c // 3
            if (rows[r] | cols[c] | boxes[b]) & bit:
                return False
            rows[r] |= bit
            cols[c] |= bit
            boxes[b] |= bit
    return True
```

## Complexity

- Time: O(81) = O(1), one pass over the fixed grid.
- Space: O(1): 27 masks.

## Pitfalls

- Trying to solve the puzzle. Validity only concerns the filled cells; an
  unsolvable grid can still be valid.
- Getting the box index wrong, e.g. `r // 3 + c // 3`, which merges
  different boxes.
- Treating `'.'` as a digit and reporting repeated empty cells.
