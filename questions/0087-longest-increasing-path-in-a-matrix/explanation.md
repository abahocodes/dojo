# Approach: DP over cells in decreasing order of height

Uphill moves form a directed acyclic graph: an edge goes from a cell to each
strictly higher neighbour, and heights strictly increase along any path, so
there are no cycles. We want the longest path in this DAG.

Let `best[r][c]` be the length of the longest route that **starts** at
`(r, c)`. It is `1` plus the largest `best` among strictly higher neighbours.
If we visit cells from the highest to the lowest, every higher neighbour has
already been finalised when we need it, so a single pass suffices and no
recursion is needed.

```python
def longest_increasing_path(matrix):
    rows, cols = len(matrix), len(matrix[0])
    order = sorted(((matrix[r][c], r, c) for r in range(rows) for c in range(cols)),
                   reverse=True)
    best = [[1] * cols for _ in range(rows)]
    answer = 1
    for height, r, c in order:
        length = 1
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and matrix[nr][nc] > height:
                length = max(length, best[nr][nc] + 1)
        best[r][c] = length
        answer = max(answer, length)
    return answer
```

**Alternatives:** memoised DFS from every cell is the most common interview
answer and has the same complexity, but its recursion depth equals the route
length. A topological sort (repeatedly peel off cells with no higher
neighbours, counting the layers) avoids both sorting and recursion and runs in
O(rows × cols).

## Complexity

- Time: O(rows × cols × log(rows × cols)) for the sort; the DP pass is linear.
- Space: O(rows × cols).

## Pitfalls

- Recursive DFS on a 200 × 200 "snake" path recurses 40,000 deep, overflowing
  Python's default limit (about 1,000) and possibly the JavaScript stack.
- Moves must be **strictly** increasing; equal heights block the route.
- Running a fresh DFS from every cell without memoisation is exponential.
- Count cells, not steps: a single cell is a route of length `1`.
