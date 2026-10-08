def num_submatrix_sum_target(matrix: list[list[int]], target: int) -> int:
    rows, cols = len(matrix), len(matrix[0])
    count = 0
    for top in range(rows):
        col = [0] * cols
        for bottom in range(top, rows):
            row = matrix[bottom]
            for c in range(cols):
                col[c] += row[c]
            seen = {0: 1}
            s = 0
            for v in col:
                s += v
                count += seen.get(s - target, 0)
                seen[s] = seen.get(s, 0) + 1
    return count
