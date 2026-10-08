# Approach: min-heap flood (minimax Dijkstra)

At time `t` you can use exactly the cells with height `<= t`. A path works at
time `t` if all its cells are `<= t`, so the answer is the minimum over all
paths of the path's highest cell.

Picture the water rising. The region you can reach grows, and it always grows
through its lowest neighbouring cell next. A min-heap simulates that:

1. Start with `(0, 0)` in the heap.
2. Pop the lowest cell. The water must reach at least its height, so update
   `level = max(level, height)`.
3. If it's the target, `level` is the answer. Otherwise push its unvisited
   neighbours.

```python
import heapq

def swim_in_water(grid):
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
```

Why it's correct: cells are popped in an order where `level` never needs to
exceed what's unavoidable. Whenever `level` rises to `h`, every reachable
cell lower than `h` has already been explored, so no path can avoid a cell of
height `h`.

**Alternatives:** binary search `t` over `0 .. n² - 1` and check with BFS
whether the target is reachable using cells `<= t` (O(n² log n)). Or sort cells
by height and add them to a union-find until `(0, 0)` and `(n-1, n-1)` are
connected.

## Complexity

- Time: O(n² log n): each cell is pushed and popped once.
- Space: O(n²) for the heap and the visited marks.

## Pitfalls

- The start cell's own height counts: if `grid[0][0]` is large, you wait for
  it too. So does the target's.
- This is not a shortest-path problem: path **length** doesn't matter, only
  the highest cell on the path.
- Mark cells as seen when you push them, so each cell is pushed only once.
- `n = 1`: the answer is `grid[0][0]`, which is `0`.
