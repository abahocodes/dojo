# Approach: search uphill from each ocean

Water flows from a cell to a neighbour that is not higher. Reversed, an ocean
"reaches" a cell if you can climb from the coast to it through neighbours that
are not lower. So instead of asking, for each cell, where its water ends up,
start at each coast and mark everything you can climb to.

Do this twice, once per ocean, with an iterative DFS seeded with all coastal
cells. A cell belongs in the answer when both searches marked it. Listing the
cells in row-major order keeps the output deterministic.

```python
def pacific_atlantic(heights):
    rows, cols = len(heights), len(heights[0])

    def reachable(starts):
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
    return [[r, c] for r in range(rows) for c in range(cols)
            if pacific[r][c] and atlantic[r][c]]
```

## Complexity

- Time: O(R · C). Each search visits each cell at most once.
- Space: O(R · C) for the two visited grids and the stack.

## Pitfalls

- Getting the comparison backwards. Going uphill from the ocean, you move to a
  neighbour that is `>=` the current cell, not `<=`.
- Using `>` instead of `>=`. Water flows across flat ground, so equal heights
  must connect.
- Running a full search from every cell: O((R · C)²), which is far too slow
  on a 200 by 200 island.
- Recursive DFS on a big plateau can exceed Python's recursion limit. Use an
  explicit stack.
- Forgetting that the top-right and bottom-left corners touch both oceans.
