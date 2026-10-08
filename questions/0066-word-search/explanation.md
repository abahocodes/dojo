# Approach: backtracking DFS from every cell

Try every cell as a starting point. From a cell that matches `word[i]`,
temporarily mark it as used and look for `word[i + 1]` among its four
neighbours. Restore the cell when backing out so other paths can use it.

Two cheap checks prune hopeless searches before they start:

- If the board doesn't contain enough copies of some letter, answer `false`
  immediately.
- If the last letter of `word` is rarer on the board than the first, search
  for the reversed word instead; a path spelled backwards is still a path,
  and starting from the rare end gives fewer starting points.

```python
from collections import Counter

def exist(board, word):
    rows, cols = len(board), len(board[0])
    on_board = Counter(ch for row in board for ch in row)
    if any(on_board[ch] < k for ch, k in Counter(word).items()):
        return False
    if on_board[word[0]] > on_board[word[-1]]:
        word = word[::-1]

    def dfs(r, c, i):
        if not (0 <= r < rows and 0 <= c < cols) or board[r][c] != word[i]:
            return False
        if i == len(word) - 1:
            return True
        board[r][c] = "#"                 # mark as used on this path
        found = (dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1)
                 or dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1))
        board[r][c] = word[i]             # restore before backing out
        return found

    return any(dfs(r, c, 0) for r in range(rows) for c in range(cols))
```

## Complexity

- Time: O(R · C · 3^L) in the worst case for an `R × C` board and a word of
  length `L`: after the first step each cell has at most 3 unvisited
  neighbours to try.
- Space: O(L) recursion depth; marking cells in place needs no extra set.

## Pitfalls

- Forgetting to restore the cell after exploring makes later paths think it is
  still taken, producing false negatives.
- Using a global `visited` set without removing cells on the way back has the
  same bug.
- Check bounds before reading `board[r][c]`; in Python negative indices
  silently wrap around.
- Return as soon as a path is found: continuing the search can be very slow.
