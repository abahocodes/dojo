import heapq


def swim_in_water(grid: list[list[int]]) -> int:
    n = len(grid)
    seen = [[False] * n for _ in range(n)]
    seen[0][0] = True
    heap = [(grid[0][0], 0, 0)]
    level = 0
    while heap:
        h, r, c = heapq.heappop(heap)
        level = max(level, h)
        if r == n - 1 and c == n - 1:
            return level
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < n and 0 <= nc < n and not seen[nr][nc]:
                seen[nr][nc] = True
                heapq.heappush(heap, (grid[nr][nc], nr, nc))
    return level
