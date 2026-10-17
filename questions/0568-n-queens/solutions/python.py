def solve_n_queens(n: int) -> list[list[str]]:
    cols, diag, anti = set(), set(), set()
    queen_col = [0] * n  # queen_col[r] = column of the queen in row r
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
