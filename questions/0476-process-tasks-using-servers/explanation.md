# Approach: two heaps and a moving clock

Keep a clock `time`. Task `j` cannot start before second `j`, nor before the
previous task was assigned, so the clock becomes `max(time, j)`. If no server
is free at that moment, the task waits: jump the clock to the earliest finish
time among busy servers.

Two min-heaps make every step logarithmic:

- `free` holds `(weight, index)`, so its top is exactly the server the rules
  pick.
- `busy` holds `(free_time, weight, index)`; its top is the next server to
  come back.

Before each assignment, pop every busy server whose free time is `<= time`
into `free`, then pop the best free server and push it onto `busy` with free
time `time + tasks[j]`.

```python
import heapq


def assign_tasks(servers, tasks):
    free = [(w, i) for i, w in enumerate(servers)]
    heapq.heapify(free)
    busy = []
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
```

## Complexity

- Time: O((n + m) log n) for `n` servers and `m` tasks; each task causes one
  push and one pop on each heap.
- Space: O(n + m) for the heaps and the answer.

## Pitfalls

- Simulating second by second is far too slow: times reach about
  `10^5 * 2 * 10^5`.
- That same bound overflows 32-bit integers, so use 64-bit time in Java and
  C++.
- When the clock jumps to the next finish time, release *all* servers that
  finish then, not just one, or ties by weight and index are decided wrongly.
- A queued task must not start earlier than the task before it; the clock
  only moves forward.
