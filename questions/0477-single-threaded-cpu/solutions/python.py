import heapq


def get_order(tasks: list[list[int]]) -> list[int]:
    n = len(tasks)
    by_enqueue = sorted(range(n), key=lambda i: (tasks[i][0], i))
    ready = []  # (processing time, index)
    order = []
    time = 0
    p = 0
    while len(order) < n:
        if not ready and time < tasks[by_enqueue[p]][0]:
            time = tasks[by_enqueue[p]][0]
        while p < n and tasks[by_enqueue[p]][0] <= time:
            i = by_enqueue[p]
            heapq.heappush(ready, (tasks[i][1], i))
            p += 1
        duration, i = heapq.heappop(ready)
        time += duration
        order.append(i)
    return order
