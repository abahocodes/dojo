# Approach: Dijkstra's algorithm

The alert reaches each server along the fastest route, so its arrival time is
the shortest-path distance from `k`. The whole network has the alert when the
farthest server gets it: the answer is the maximum distance, or `-1` if any
distance is infinite.

All weights are non-negative, so Dijkstra works: always settle the unsettled
server with the smallest known time. A min-heap provides it. Rather than
decreasing keys in place, push a new entry whenever a shorter time is found
and skip stale entries when they are popped ("lazy deletion").

```python
import heapq

def network_delay_time(times, n, k):
    graph = [[] for _ in range(n + 1)]
    for u, v, w in times:
        graph[u].append((v, w))

    dist = [None] * (n + 1)
    heap = [(0, k)]
    while heap:
        d, node = heapq.heappop(heap)
        if dist[node] is not None:
            continue
        dist[node] = d
        for nxt, w in graph[node]:
            if dist[nxt] is None:
                heapq.heappush(heap, (d + w, nxt))

    if any(dist[i] is None for i in range(1, n + 1)):
        return -1
    return max(dist[1:])
```

**Alternative:** Bellman-Ford relaxes every link up to `n - 1` times. It is
simpler and also handles negative weights, but costs O(V · E).

## Complexity

- Time: O(E log E) for the heap operations (each link pushes at most once).
- Space: O(V + E) for the adjacency list, distances and heap.

## Pitfalls

- Servers are numbered from `1`. Size the arrays `n + 1` and skip index `0`
  when taking the maximum.
- Settling a server when it is *pushed* rather than when it is *popped*. The
  first push is not necessarily the shortest time.
- Using plain BFS. It ignores weights, so a two-hop route can beat a one-hop
  route (as in Example 1) and BFS would miss it.
- Links are one-way. Adding the reverse edge changes the answer.
