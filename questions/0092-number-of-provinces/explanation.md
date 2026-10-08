# Approach: union-find over the adjacency matrix

The matrix describes an undirected graph, and a province is a connected
component, so the task is to count components.

Union-find (disjoint set union) does this directly. Every town starts in its
own set, so there are `n` sets. Walk the upper triangle of the matrix (the
lower half is a mirror and the diagonal is always `1`). For each road, find
the representative (root) of both towns. If the roots differ, the road joins
two separate provinces: link one root under the other and decrease the count.
If they're equal, the road adds nothing new.

```python
def find_circle_num(is_connected):
    n = len(is_connected)
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]  # path halving
            x = parent[x]
        return x

    provinces = n
    for i in range(n):
        for j in range(i + 1, n):
            if is_connected[i][j] == 1:
                ri, rj = find(i), find(j)
                if ri != rj:
                    parent[ri] = rj
                    provinces -= 1
    return provinces
```

**Alternative:** DFS or BFS. Keep a `visited` array; for every unvisited town,
add one to the count and explore everything reachable from it by scanning its
row of the matrix. Same O(n²) time.

## Complexity

- Time: O(n² · α(n)), effectively O(n²): reading the matrix dominates.
- Space: O(n) for the parent array.

## Pitfalls

- The input is an **adjacency matrix**, not an edge list. Row `i` lists
  neighbours of town `i` by position, so don't treat `is_connected[i]` as a
  pair of towns.
- Count merges, not roads. A road between two towns already in the same
  province must not reduce the count.
- Without path compression (or union by rank), `find` can walk long chains;
  it's still correct, just slower.
