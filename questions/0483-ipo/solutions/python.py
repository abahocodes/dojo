import heapq


def find_maximized_capital(k: int, w: int, profits: list[int], capital: list[int]) -> int:
    projects = sorted(zip(capital, profits))
    affordable = []  # max-heap of profits (stored negated)
    p = 0
    for _ in range(k):
        while p < len(projects) and projects[p][0] <= w:
            heapq.heappush(affordable, -projects[p][1])
            p += 1
        if not affordable:
            break
        w -= heapq.heappop(affordable)
    return w
