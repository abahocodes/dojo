def pacific_atlantic(heights: list[list[int]]) -> list[list[int]]:
    rows, cols = len(heights), len(heights[0])

    def reachable(starts):
        # Walk uphill from the ocean: water can flow from each reached cell
        # down to the ocean. Iterative DFS keeps large grids off the call stack.
        seen = [[False] * cols for _ in range(rows)]
        stack = []
        for r, c in starts:
            if not seen[r][c]:
                seen[r][c] = True
                stack.append((r, c))
        while stack:
            r, c = stack.pop()
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if (0 <= nr < rows and 0 <= nc < cols and not seen[nr][nc]
                        and heights[nr][nc] >= heights[r][c]):
                    seen[nr][nc] = True
                    stack.append((nr, nc))
        return seen

    pacific = reachable([(0, c) for c in range(cols)] + [(r, 0) for r in range(rows)])
    atlantic = reachable([(rows - 1, c) for c in range(cols)] + [(r, cols - 1) for r in range(rows)])
    return [[r, c] for r in range(rows) for c in range(cols) if pacific[r][c] and atlantic[r][c]]
