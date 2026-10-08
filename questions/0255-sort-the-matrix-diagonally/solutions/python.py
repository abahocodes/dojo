def diagonal_sort(mat: list[list[int]]) -> list[list[int]]:
    m, n = len(mat), len(mat[0])
    res = [row[:] for row in mat]
    starts = [(i, 0) for i in range(m)] + [(0, j) for j in range(1, n)]
    for si, sj in starts:
        length = min(m - si, n - sj)
        values = sorted(res[si + k][sj + k] for k in range(length))
        for k in range(length):
            res[si + k][sj + k] = values[k]
    return res
