def longest_increasing_path(matrix: list[list[int]]) -> int:
    rows, cols = len(matrix), len(matrix[0])
    order = sorted(((matrix[r][c], r, c) for r in range(rows) for c in range(cols)),
                   reverse=True)
    best = [[1] * cols for _ in range(rows)]
    answer = 1
    for height, r, c in order:
        length = 1
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and matrix[nr][nc] > height:
                if best[nr][nc] + 1 > length:
                    length = best[nr][nc] + 1
        best[r][c] = length
        if length > answer:
            answer = length
    return answer
