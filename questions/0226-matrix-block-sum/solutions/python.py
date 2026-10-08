def matrix_block_sum(mat: list[list[int]], k: int) -> list[list[int]]:
    m, n = len(mat), len(mat[0])
    pre = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(m):
        for j in range(n):
            pre[i + 1][j + 1] = mat[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j]
    result = [[0] * n for _ in range(m)]
    for i in range(m):
        r1, r2 = max(0, i - k), min(m, i + k + 1)
        for j in range(n):
            c1, c2 = max(0, j - k), min(n, j + k + 1)
            result[i][j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1]
    return result
