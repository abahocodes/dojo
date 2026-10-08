# Approach: greedy with a sorted list and a max-heap

Since capital only grows, the set of affordable projects only grows too. At
each step, picking the most profitable affordable project is optimal: any plan
that picks a smaller profit now can swap it for the larger one, ending with at
least as much capital at every later step.

Sort projects by required capital. Keep a pointer into that order and a
max-heap of profits:

1. Push the profit of every project whose required capital is `<= w`.
2. If the heap is empty, nothing more can be done; stop.
3. Otherwise pop the largest profit and add it to `w`.

Repeat at most `k` times.

```python
import heapq


def find_maximized_capital(k, w, profits, capital):
    projects = sorted(zip(capital, profits))
    affordable = []
    p = 0
    for _ in range(k):
        while p < len(projects) and projects[p][0] <= w:
            heapq.heappush(affordable, -projects[p][1])
            p += 1
        if not affordable:
            break
        w -= heapq.heappop(affordable)
    return w
```

## Complexity

- Time: O(n log n + k log n).
- Space: O(n).

## Pitfalls

- Rescanning all projects every round is O(nk), too slow for `10^5` each.
- Stop when no project is affordable instead of looping uselessly through the
  remaining rounds.
- Projects with zero profit are allowed; they never hurt, but they never help.
- The required capital is not subtracted from `w`.
