def oranges_rotting(grid: list[list[int]]) -> int:
    rows, cols = len(grid), len(grid[0])
    state = [row[:] for row in grid]  # don't mutate the caller's grid
    frontier = []
    fresh = 0
    for r in range(rows):
        for c in range(cols):
            if state[r][c] == 2:
                frontier.append((r, c))
            elif state[r][c] == 1:
                fresh += 1

    minutes = 0
    while frontier and fresh > 0:
        minutes += 1
        nxt = []
        for r, c in frontier:
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < rows and 0 <= nc < cols and state[nr][nc] == 1:
                    state[nr][nc] = 2
                    fresh -= 1
                    nxt.append((nr, nc))
        frontier = nxt
    return minutes if fresh == 0 else -1
