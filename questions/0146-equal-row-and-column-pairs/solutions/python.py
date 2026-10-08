def equal_pairs(grid: list[list[int]]) -> int:
    rows = {}
    for row in grid:
        key = tuple(row)
        rows[key] = rows.get(key, 0) + 1
    return sum(rows.get(tuple(col), 0) for col in zip(*grid))
