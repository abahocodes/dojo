import heapq


def assign_tasks(servers: list[int], tasks: list[int]) -> list[int]:
    free = [(w, i) for i, w in enumerate(servers)]
    heapq.heapify(free)
    busy = []  # (time it becomes free, weight, index)
    result = []
    time = 0
    for j, duration in enumerate(tasks):
        time = max(time, j)
        if not free:
            time = max(time, busy[0][0])
        while busy and busy[0][0] <= time:
            _, w, i = heapq.heappop(busy)
            heapq.heappush(free, (w, i))
        w, i = heapq.heappop(free)
        result.append(i)
        heapq.heappush(busy, (time + duration, w, i))
    return result
