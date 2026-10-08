import heapq


def find_order(num_courses: int, prerequisites: list[list[int]]) -> list[int]:
    unlocks = [[] for _ in range(num_courses)]
    indegree = [0] * num_courses
    for course, before in prerequisites:
        unlocks[before].append(course)
        indegree[course] += 1

    # A min-heap always hands out the smallest course that is ready now,
    # which gives the lexicographically smallest valid order.
    ready = [i for i in range(num_courses) if indegree[i] == 0]
    heapq.heapify(ready)
    order = []
    while ready:
        current = heapq.heappop(ready)
        order.append(current)
        for nxt in unlocks[current]:
            indegree[nxt] -= 1
            if indegree[nxt] == 0:
                heapq.heappush(ready, nxt)
    return order if len(order) == num_courses else []
