# Approach: multi-source BFS, one level per minute

The minute a fresh orange rots equals its grid distance to the *nearest*
rotten orange (moving through oranges only). A BFS seeded with every rotten
orange at distance `0` computes exactly these distances, level by level. The
answer is the number of levels it takes to reach every fresh orange.

Process the BFS one whole frontier at a time so that each round is one minute,
and keep a count of fresh oranges so you know at the end whether any were
unreachable.

```python
def oranges_rotting(grid):
    rows, cols = len(grid), len(grid[0])
    state = [row[:] for row in grid]
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
```

## Complexity

- Time: O(R · C). Every cell enters the frontier at most once.
- Space: O(R · C) for the copy of the grid and the frontier.

## Pitfalls

- Running a separate BFS from each rotten orange and taking the maximum. That
  answers a different question and is O((R · C)²). Seed all sources at once.
- Counting one minute too many: the loop must stop as soon as nothing fresh
  remains, not when the frontier finally runs dry.
- Returning `-1` for a grid with no fresh oranges. Nothing needs to rot, so the
  answer is `0`, even if there are no rotten oranges either.
- Marking a cell rotten only when it is popped instead of when it is pushed
  lets the same orange be queued twice.
