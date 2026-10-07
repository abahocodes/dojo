from collections import deque


def can_finish(num_courses: int, prerequisites: list[list[int]]) -> bool:
    unlocks = [[] for _ in range(num_courses)]
    indegree = [0] * num_courses
    for course, before in prerequisites:
        unlocks[before].append(course)
        indegree[course] += 1

    ready = deque(i for i in range(num_courses) if indegree[i] == 0)
    taken = 0
    while ready:
        current = ready.popleft()
        taken += 1
        for nxt in unlocks[current]:
            indegree[nxt] -= 1
            if indegree[nxt] == 0:
                ready.append(nxt)
    return taken == num_courses
