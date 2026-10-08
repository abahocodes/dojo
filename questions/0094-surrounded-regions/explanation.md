# Approach: flood fill from the border

Checking each region separately for a border cell works, but it's simpler to
flip the question. A region escapes capture exactly when it contains a border
cell, so every surviving `"O"` is reachable from some border `"O"`.

1. Push every `"O"` on the border onto a stack and mark it safe.
2. Flood fill: pop a cell, and push each `"O"` neighbour that isn't marked
   yet, marking it safe.
3. Build the result: safe cells are `"O"`, all others are `"X"`.

Every `"O"` that was never marked belongs to a region with no border cell,
so it is captured.

```python
def capture_regions(board):
    rows, cols = len(board), len(board[0])
    safe = [[False] * cols for _ in range(rows)]
    stack = []
    for r in range(rows):
        for c in range(cols):
            on_edge = r in (0, rows - 1) or c in (0, cols - 1)
            if on_edge and board[r][c] == "O":
                safe[r][c] = True
                stack.append((r, c))
    while stack:
        r, c = stack.pop()
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols \
                    and board[nr][nc] == "O" and not safe[nr][nc]:
                safe[nr][nc] = True
                stack.append((nr, nc))
    return [["O" if safe[r][c] else "X" for c in range(cols)]
            for r in range(rows)]
```

## Complexity

- Time: O(rows × cols): each cell is marked at most once.
- Space: O(rows × cols) for the marks, the stack and the output.

## Pitfalls

- Only up/down/left/right count. An `"O"` that touches a safe cell
  diagonally is still captured.
- A recursive DFS can go rows × cols deep on a winding region and overflow the
  call stack. Use an explicit stack or a queue.
- Mark cells when you push them, not when you pop them, or the same cell gets
  pushed many times.
- One-row and one-column boards: every cell is on the border, so nothing is
  captured.
