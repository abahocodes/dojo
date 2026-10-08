# Approach: sweep line with a max-heap

The outline height can only change at a building's `left` or `right`, so we
sweep over those coordinates in order. Each building contributes two events:

- `(left, -height, right)`: the building starts;
- `(right, 0, 0)`: a reminder to re-check the height when it ends.

Sorting the tuples orders events by `x`, puts starts (negative second field)
before ends at the same `x`, and puts taller starts first.

A max-heap holds `(height, right)` for every building that might still stand.
We never delete a building when it ends. Instead, before reading the top, we
pop tops whose `right <= x`: those buildings no longer cover `x`. Buried
expired entries are harmless because they are not the maximum; they are
discarded later when they surface.

After handling an event at `x`, the heap top (or `0` if the heap is empty) is
the outline height starting at `x`. We append `[x, height]` only if the height
differs from the previous key point. Because the first event at any `x` already
pops every building ending at `x` and pushes the tallest one starting there,
later events at the same `x` never change the top, so no `x` gets two points.

```python
import heapq

def get_skyline(buildings):
    events = []
    for left, right, height in buildings:
        events.append((left, -height, right))
        events.append((right, 0, 0))
    events.sort()

    result = []
    live = []  # (-height, right)
    for x, neg_height, right in events:
        while live and live[0][1] <= x:
            heapq.heappop(live)
        if neg_height:
            heapq.heappush(live, (neg_height, right))
        current = -live[0][0] if live else 0
        if not result or result[-1][1] != current:
            result.append([x, current])
    return result
```

## Complexity

- Time: O(n log n) for sorting the `2n` events and the heap operations.
- Space: O(n) for the events and the heap.

## Pitfalls

- Processing ends before starts at the same `x`. With `[0,3,4]` and `[3,6,4]`
  you would briefly drop to `0` at `x = 3` and emit `[3,0]`, `[3,4]`.
- Emitting a point for every event. Hidden buildings and equal-height
  neighbours would produce repeated heights.
- A ground sentinel such as `(0, INT_MAX)` in a 32-bit language: a building
  may end at exactly `2^31 - 1`, which would expire the sentinel. Treat an
  empty heap as height `0` instead.
- Forgetting the final `[right, 0]` point, or emitting `[x, 0]` in a gap only
  once the next building starts.
