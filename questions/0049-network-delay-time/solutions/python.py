import heapq


def network_delay_time(times: list[list[int]], n: int, k: int) -> int:
    graph = [[] for _ in range(n + 1)]
    for u, v, w in times:
        graph[u].append((v, w))

    dist = [None] * (n + 1)
    heap = [(0, k)]
    while heap:
        d, node = heapq.heappop(heap)
        if dist[node] is not None:
            continue  # stale entry: already settled with a smaller time
        dist[node] = d
        for nxt, w in graph[node]:
            if dist[nxt] is None:
                heapq.heappush(heap, (d + w, nxt))

    if any(dist[i] is None for i in range(1, n + 1)):
        return -1
    return max(dist[1:])
