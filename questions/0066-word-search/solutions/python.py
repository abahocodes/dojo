from collections import Counter

def exist(board: list[list[str]], word: str) -> bool:
    rows, cols = len(board), len(board[0])
    on_board = Counter(ch for row in board for ch in row)
    if any(on_board[ch] < k for ch, k in Counter(word).items()):
        return False
    # a path read backwards is still a path; start from the rarer end to prune sooner
    if on_board[word[0]] > on_board[word[-1]]:
        word = word[::-1]

    def dfs(r, c, i):
        if not (0 <= r < rows and 0 <= c < cols) or board[r][c] != word[i]:
            return False
        if i == len(word) - 1:
            return True
        board[r][c] = "#"  # mark as used on the current path
        found = (dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1)
                 or dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1))
        board[r][c] = word[i]
        return found

    return any(dfs(r, c, 0) for r in range(rows) for c in range(cols))
