def region_sums(matrix: list[list[int]], queries: list[list[int]]) -> list[int]:
    m, n = len(matrix), len(matrix[0])
    # pre[i][j] = sum of matrix[0..i-1][0..j-1]
    pre = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(m):
        for j in range(n):
            pre[i + 1][j + 1] = matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j]
    return [
        pre[r2 + 1][c2 + 1] - pre[r1][c2 + 1] - pre[r2 + 1][c1] + pre[r1][c1]
        for r1, c1, r2, c2 in queries
    ]
