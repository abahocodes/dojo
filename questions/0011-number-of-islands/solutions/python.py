def num_islands(grid: list[list[str]]) -> int:
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    seen = [[False] * cols for _ in range(rows)]
    count = 0
    for r in range(rows):
        for c in range(cols):
            if grid[r][c] != "1" or seen[r][c]:
                continue
            count += 1
            seen[r][c] = True
            stack = [(r, c)]
            while stack:
                i, j = stack.pop()
                for ni, nj in ((i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)):
                    if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == "1" and not seen[ni][nj]:
                        seen[ni][nj] = True
                        stack.append((ni, nj))
    return count
