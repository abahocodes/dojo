# Approach: max-heap simulation

Each turn needs the two largest values, and then possibly inserts one new
value. A max-heap does both operations in O(log n), so the simulation is
straightforward. Every turn removes at least one stone, so there are fewer
than `n` turns.

Python's `heapq` is a min-heap; storing negated weights turns it into a
max-heap.

```python
import heapq

def last_stone_weight(stones):
    heap = [-s for s in stones]
    heapq.heapify(heap)
    while len(heap) > 1:
        y = -heapq.heappop(heap)
        x = -heapq.heappop(heap)
        if y != x:
            heapq.heappush(heap, -(y - x))
    return -heap[0] if heap else 0
```

**Alternative (counting):** weights are at most 1000, so you can keep a count
per weight and scan downward from the heaviest weight. That gives
O(n + W) time with `W = 1000`.

## Complexity

- Time: O(n log n): heapify is O(n) and each of fewer than `n` turns does a
  constant number of heap operations.
- Space: O(n) for the heap.

## Pitfalls

- Pop the heaviest stone first: `y - x` must be non-negative.
- When the two stones are equal, push nothing (pushing a `0` stone would be
  smashed later and change nothing, but it is easy to then return `0` for the
  wrong reason or loop forever on a buggy condition).
- Return `0` for an empty pile, not an error from reading the top of an empty
  heap.
