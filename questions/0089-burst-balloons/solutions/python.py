def max_coins(nums: list[int]) -> int:
    v = [1] + nums + [1]
    n = len(v)
    best = [[0] * n for _ in range(n)]
    for length in range(2, n):
        for i in range(n - length):
            j = i + length
            edge = v[i] * v[j]
            row_i = best[i]
            top = 0
            for k in range(i + 1, j):
                total = row_i[k] + edge * v[k] + best[k][j]
                if total > top:
                    top = total
            row_i[j] = top
    return best[0][n - 1]
