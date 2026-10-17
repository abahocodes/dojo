# Approach: flood fill each island and count its cells

Walk over the grid. The first time you meet a land cell that no earlier fill
has reached, it is the start of a new island. Flood fill from it, visiting
edge-adjacent land cells, and count how many cells the fill touches. That
count is the island's area; keep the largest one.

A `seen` matrix (or overwriting visited land with `0`, if you may change the
input) ensures every cell is counted once, by exactly one fill.

```python
def max_area_of_island(grid):
    rows, cols = len(grid), len(grid[0])
    seen = [[False] * cols for _ in range(rows)]
    best = 0
    for r in range(rows):
        for c in range(cols):
            if grid[r][c] != 1 or seen[r][c]:
                continue
            seen[r][c] = True
            stack, area = [(r, c)], 0
            while stack:
                y, x = stack.pop()
                area += 1
                for ny, nx in ((y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)):
                    if 0 <= ny < rows and 0 <= nx < cols \
                            and grid[ny][nx] == 1 and not seen[ny][nx]:
                        seen[ny][nx] = True
                        stack.append((ny, nx))
            best = max(best, area)
    return best
```

## Complexity

- Time: O(rows × cols). Every cell is pushed at most once and looks at four
  neighbours.
- Space: O(rows × cols) for `seen` and, in the worst case, the stack.

## Pitfalls

- Mark a cell as seen when you **push** it, not when you pop it. Otherwise
  the same cell can sit on the stack several times and be counted twice.
- Diagonal neighbours are not connected; only check the four edge directions.
- A recursive DFS on a 50 × 50 all-land grid recurses 2,500 levels deep,
  which is past Python's default limit. Use an explicit stack or BFS queue.
- An all-water grid has answer `0`, not an error; start `best` at `0`.
