def maximal_square(matrix: list[list[str]]) -> int:
    cols = len(matrix[0])
    side = [0] * (cols + 1)
    best = 0
    for row in matrix:
        prev_diag = 0
        for c in range(1, cols + 1):
            above = side[c]
            if row[c - 1] == "1":
                side[c] = 1 + min(above, side[c - 1], prev_diag)
                if side[c] > best:
                    best = side[c]
            else:
                side[c] = 0
            prev_diag = above
    return best * best
