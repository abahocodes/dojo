# Hints

## Hint 1
This is "count the islands" with one extra number per island. When you step
onto a land cell you haven't visited yet, you have found a new island; how
would you visit all of it, and only it?

## Hint 2
Flood fill from that cell (DFS or BFS) over edge-adjacent land. Every cell
the fill reaches belongs to the island, so count them as you go. Mark cells
as visited when you first reach them so no cell is counted twice.

## Hint 3
Scan every cell. For each unvisited `1`, run the flood fill, which returns
the island's size, and keep the maximum. Each cell is visited at most once,
so the whole scan is O(rows × cols). Use an explicit stack if you're worried
about recursion depth on a 2,500-cell island.
