import heapq


def trap_rain_water_2d(height_map: list[list[int]]) -> int:
    m, n = len(height_map), len(height_map[0])
    visited = [[False] * n for _ in range(m)]
    heap = []  # (water level, row, col) on the frontier
    for r in range(m):
        for c in range(n):
            if r == 0 or c == 0 or r == m - 1 or c == n - 1:
                heapq.heappush(heap, (height_map[r][c], r, c))
                visited[r][c] = True

    total = 0
    while heap:
        level, r, c = heapq.heappop(heap)
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < m and 0 <= nc < n and not visited[nr][nc]:
                visited[nr][nc] = True
                h = height_map[nr][nc]
                if h < level:
                    total += level - h
                heapq.heappush(heap, (max(h, level), nr, nc))
    return total
