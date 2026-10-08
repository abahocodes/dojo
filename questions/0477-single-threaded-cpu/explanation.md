# Approach: sort by arrival, then a heap of ready tasks

Sort the indices by `(enqueue_time, index)` so tasks can be released into a
"ready" pool in arrival order with a single pointer. The ready pool is a
min-heap on `(processing_time, index)`, which encodes the CPU's choice rule
directly.

Loop until every task is processed:

1. If nothing is ready and the clock is before the next arrival, jump the
   clock to that arrival (the CPU idles).
2. Push every task with `enqueue_time <= clock` into the heap.
3. Pop the best task, record it, and advance the clock by its processing time.

```python
import heapq


def get_order(tasks):
    n = len(tasks)
    by_enqueue = sorted(range(n), key=lambda i: (tasks[i][0], i))
    ready = []
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
```

## Complexity

- Time: O(n log n) for the sort plus one push and pop per task.
- Space: O(n).

## Pitfalls

- The clock can reach about `10^5 * 10^9`, so it needs 64 bits in Java/C++.
- Only jump the clock forward when the heap is empty; otherwise a task that
  arrives later could wrongly be considered.
- The heap key must include the index so ties on processing time go to the
  smaller index, not to whichever arrived first.
