def find_words(board: list[list[str]], words: list[str]) -> list[str]:
    root = {}
    for word in words:
        node = root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = word

    rows, cols = len(board), len(board[0])
    found = []

    def dfs(r: int, c: int, parent: dict) -> None:
        ch = board[r][c]
        node = parent.get(ch)
        if node is None:
            return
        word = node.pop("$", None)
        if word is not None:
            found.append(word)
        board[r][c] = "#"
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] != "#":
                dfs(nr, nc, node)
        board[r][c] = ch
        if not node:
            del parent[ch]  # prune branches with nothing left to find

    for r in range(rows):
        for c in range(cols):
            dfs(r, c, root)
    return found
