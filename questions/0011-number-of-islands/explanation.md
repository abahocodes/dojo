# Approach: flood fill each new island

Treat the grid as a graph whose nodes are land cells, with an edge between
cells that share a side. The answer is the number of connected components.

Scan the grid. The first time you see an unvisited land cell, it starts a new
island: increment the count, then flood fill from it to mark the whole island
as visited. An explicit stack avoids recursion-depth problems on large islands.

```python
def num_islands(grid):
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
                    if 0 <= ni < rows and 0 <= nj < cols \
                            and grid[ni][nj] == "1" and not seen[ni][nj]:
                        seen[ni][nj] = True
                        stack.append((ni, nj))
    return count
```

**Alternative:** union-find. Union each land cell with its land neighbours to
the right and below, then count distinct roots among land cells. This works
well when cells arrive one at a time (an "online" version of the problem).

## Complexity

- Time: O(rows × cols): each cell is pushed at most once and looks at 4
  neighbours.
- Space: O(rows × cols) for the visited marks and the stack in the worst case.

## Pitfalls

- The cells are the **strings** `"1"` and `"0"`, not integers. Comparing with
  `1` is always false.
- Mark a cell visited when you **push** it, not when you pop it. Otherwise the
  same cell can be pushed many times.
- A recursive DFS on a grid that is almost all land can recurse rows × cols
  deep, which overflows the stack in many languages.
- Many solutions overwrite land with `"0"` instead of keeping a `seen` grid.
  That's fine in an interview, but say out loud that it mutates the input.
