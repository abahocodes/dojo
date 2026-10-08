def is_valid_sudoku(board: list[str]) -> bool:
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
