# Approach: multi-source BFS from all gates

Imagine all gates "flooding" outwards at the same speed, one step per round.
The first wave to reach a room comes from its nearest gate, and the round
number is the distance. That's exactly a breadth-first search that starts with
every gate already in the queue.

The value `2147483647` doubles as the "not visited yet" marker: a room is
filled the first time BFS reaches it and is never touched again.

```python
from collections import deque

EMPTY = 2147483647

def walls_and_gates(rooms):
    rows, cols = len(rooms), len(rooms[0])
    dist = [row[:] for row in rooms]
    queue = deque((r, c) for r in range(rows) for c in range(cols)
                  if dist[r][c] == 0)
    while queue:
        r, c = queue.popleft()
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and dist[nr][nc] == EMPTY:
                dist[nr][nc] = dist[r][c] + 1
                queue.append((nr, nc))
    return dist
```

Walls are skipped automatically because `-1 != EMPTY`, and gates because
`0 != EMPTY`.

## Complexity

- Time: O(rows × cols): every cell is queued at most once.
- Space: O(rows × cols) for the queue and the output copy.

## Pitfalls

- A separate BFS per gate is O(gates × rows × cols), which is far too slow
  when there are many gates.
- If you use DFS instead of BFS, the first time you reach a room isn't
  necessarily via the shortest path, so you'd have to keep overwriting.
- In Python, `list.pop(0)` is O(n). Use `collections.deque`.
- Unreachable rooms must stay `2147483647`, not become `-1` or some other
  sentinel.
