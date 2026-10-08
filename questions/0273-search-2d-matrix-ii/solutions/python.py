def search_matrix_sorted(matrix: list[list[int]], target: int) -> bool:
    rows, cols = len(matrix), len(matrix[0])
    r, c = 0, cols - 1
    while r < rows and c >= 0:
        value = matrix[r][c]
        if value == target:
            return True
        if value > target:
            c -= 1
        else:
            r += 1
    return False
