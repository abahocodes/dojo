# Approach: shrink the boundary from its lowest point (min-heap)

Water in a square ends at the level of the lowest "spill height" along any
path to the border, where a path's spill height is the tallest square on it.
Computing this per square directly is expensive, but we can grow the answer
inward from the border, the same way Dijkstra grows shortest distances.

Keep a boundary of visited squares in a min-heap keyed by their effective
level (ground height, or water surface if water stands there). Initially the
boundary is every border square at its own height, since border squares drain.

Repeatedly pop the lowest boundary square, at level `level`. It is the weakest
part of the wall around everything still unvisited, so any unvisited
neighbour can hold water up to exactly `level` and no higher: if its ground is
lower, it traps `level - height` units. The neighbour then becomes part of the
boundary with level `max(level, height)`.

```python
import heapq

def trap_rain_water_2d(height_map):
    m, n = len(height_map), len(height_map[0])
    visited = [[False] * n for _ in range(m)]
    heap = []
    for r in range(m):
        for c in range(n):
            if r in (0, m - 1) or c in (0, n - 1):
                heapq.heappush(heap, (height_map[r][c], r, c))
                visited[r][c] = True

    total = 0
    while heap:
        level, r, c = heapq.heappop(heap)
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < m and 0 <= nc < n and not visited[nr][nc]:
                visited[nr][nc] = True
                h = height_map[nr][nc]
                total += max(0, level - h)
                heapq.heappush(heap, (max(h, level), nr, nc))
    return total
```

## Complexity

- Time: O(mn log(mn)): each square is pushed and popped once.
- Space: O(mn) for the visited grid and the heap.

## Pitfalls

- Extending the 1-D two-pointer trick (min of left and right maxima) to rows
  and columns separately. Water can escape around corners through a winding
  path, so row and column maxima overestimate the level.
- Pushing a neighbour with its own height instead of `max(height, level)`.
  A flooded square acts as a wall at the water surface, not at the ground.
- Marking squares visited when popped rather than when pushed, which lets a
  square be counted twice.
- Grids with fewer than three rows or columns have no interior and trap
  nothing; the algorithm handles them without special cases.
