from collections import deque

EMPTY = 2147483647


def walls_and_gates(rooms: list[list[int]]) -> list[list[int]]:
    rows, cols = len(rooms), len(rooms[0])
    dist = [row[:] for row in rooms]
    queue = deque((r, c) for r in range(rows) for c in range(cols) if dist[r][c] == 0)
    while queue:
        r, c = queue.popleft()
        d = dist[r][c] + 1
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and dist[nr][nc] == EMPTY:
                dist[nr][nc] = d
                queue.append((nr, nc))
    return dist
