# Approach: topological sort (Kahn's algorithm)

Model each prerequisite `[a, b]` as a directed edge `b -> a`. Every course can
be finished exactly when this graph has no directed cycle, that is, when it
has a topological order.

Kahn's algorithm builds that order greedily. Any course with in-degree `0`
(no unmet prerequisites) can be taken now. Taking it removes its outgoing
edges, which may free up more courses. If the process stalls before every
course is taken, the leftover courses all sit on or behind a cycle.

```python
from collections import deque

def can_finish(num_courses, prerequisites):
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
```

**Alternative:** DFS with three colours (unvisited / on the current path /
done). Reaching a node that is still "on the current path" means you found a
back edge, which is a cycle. It's the same complexity, but it recurses, so a
long chain of prerequisites can get deep.

## Complexity

- Time: O(V + E), where V is `num_courses` and E is `len(prerequisites)`.
- Space: O(V + E) for the adjacency list and in-degree counts.

## Pitfalls

- Getting the edge direction backwards. With in-degree counting it still gives
  the right yes/no answer, but it's a sign the model is muddled, and it matters
  as soon as you are asked for the actual order.
- A self-loop `[a, a]` is a cycle of length 1: the answer is `false`.
- Courses with no prerequisites at all still need to be counted. Don't build
  the graph only from nodes that appear in `prerequisites`.
- A two-colour DFS (just "visited") can't tell a cycle from two paths that
  meet at the same node, so it reports false cycles.
