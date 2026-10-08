# Approach: union-find, stop at the first cycle

`n` towns and `n` roads that connect everything means the graph is a tree plus
one road, so it has exactly one cycle. Removing any road on that cycle leaves
a tree; removing any other road disconnects something. We want the cycle road
that appears last in `edges`.

Add the roads in the given order, tracking connectivity with union-find. While
no cycle exists, every road joins two different groups. The first road whose
endpoints are already in the same group closes the cycle. Every other road of
the cycle was needed to connect those endpoints, so they were all added
earlier: this road is the last cycle road in the list.

```python
def find_redundant_connection(edges):
    n = len(edges)
    parent = list(range(n + 1))
    size = [1] * (n + 1)

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return [a, b]
        if size[ra] < size[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        size[ra] += size[rb]
    return []
```

**Alternative:** for each road from last to first, remove it and check with a
BFS whether the rest is still connected. That is O(n²), fine for 1000 towns
but much slower than union-find.

## Complexity

- Time: O(n · α(n)), effectively linear.
- Space: O(n) for the union-find arrays.

## Pitfalls

- Towns are numbered from `1`, so size the arrays `n + 1`.
- Returning the first road on the cycle instead of the last. The road that
  *triggers* the cycle in union-find is already the right answer; don't go
  looking for an earlier one.
- Returning `[b, a]` or a sorted pair you built yourself. Return the road as
  it was given.
