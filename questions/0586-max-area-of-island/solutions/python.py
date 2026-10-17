def max_area_of_island(grid: list[list[int]]) -> int:
    rows = len(grid)
    cols = len(grid[0]) if rows else 0
    seen = [[False] * cols for _ in range(rows)]
    best = 0
    for r in range(rows):
        for c in range(cols):
            if grid[r][c] != 1 or seen[r][c]:
                continue
            # Iterative DFS: count every land cell reachable from (r, c).
            seen[r][c] = True
            stack = [(r, c)]
            area = 0
            while stack:
                y, x = stack.pop()
                area += 1
                for ny, nx in ((y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)):
                    if 0 <= ny < rows and 0 <= nx < cols and grid[ny][nx] == 1 and not seen[ny][nx]:
                        seen[ny][nx] = True
                        stack.append((ny, nx))
            best = max(best, area)
    return best
