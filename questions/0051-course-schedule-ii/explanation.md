# Approach: Kahn's algorithm with a min-heap

Model each prerequisite `[a, b]` as a directed edge `b -> a`. A valid order is
a topological order of this graph, and one exists exactly when the graph has
no cycle.

Kahn's algorithm repeatedly takes a course whose in-degree (count of unmet
prerequisites) is `0`. When several are ready, any choice still leads to a
valid order. Picking the **smallest** ready course every time gives the
lexicographically smallest order: the first position gets the smallest course
that could possibly go first, and the same argument repeats for every later
position. A min-heap makes "smallest ready course" cheap.

```python
import heapq

def find_order(num_courses, prerequisites):
    unlocks = [[] for _ in range(num_courses)]
    indegree = [0] * num_courses
    for course, before in prerequisites:
        unlocks[before].append(course)
        indegree[course] += 1

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
```

JavaScript has no built-in heap, so the reference solution includes a small
binary heap. A linear scan for the smallest ready course also works but costs
O(V²).

## Complexity

- Time: O((V + E) + V log V), where V is `num_courses` and E is
  `len(prerequisites)`.
- Space: O(V + E) for the adjacency list, in-degrees and heap.

## Pitfalls

- Using a plain FIFO queue. It returns *a* valid order, but usually not the
  lexicographically smallest one.
- A DFS-based topological sort (reverse post-order) is also valid in general
  but doesn't produce the smallest order.
- Getting the edge direction backwards produces an order where each course
  comes *before* its prerequisites.
- Returning the partial order when a cycle exists. If fewer than `num_courses`
  courses were taken, the answer is `[]`.
- A self-loop `[a, a]` is a cycle: the answer is `[]`.
