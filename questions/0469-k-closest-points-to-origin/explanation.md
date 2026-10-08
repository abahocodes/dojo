# Approach: bounded max-heap

Compare points by squared distance `x² + y²`; it preserves the order of real
distances and stays in integers (at most `2 · 10^8`, which fits in 32 bits).

Scan the points while maintaining a max-heap of the `k` closest seen so far.
The heap's top is the farthest of those `k`. A new point only matters if it is
closer than that top, in which case it evicts it. When the scan ends, the heap
holds exactly the `k` closest points.

```python
import heapq


def k_closest(points, k):
    heap = []  # (-squared distance, index): heapq is a min-heap
    for i, (x, y) in enumerate(points):
        d = x * x + y * y
        if len(heap) < k:
            heapq.heappush(heap, (-d, i))
        elif d < -heap[0][0]:
            heapq.heapreplace(heap, (-d, i))
    return [points[i] for _, i in heap]
```

**Alternative (quickselect):** partition the points around a random pivot
distance, recursing only into the side that contains position `k`. This runs
in `O(n)` on average but `O(n²)` in the worst case, and it reorders the input.

## Complexity

- Time: `O(n log k)`.
- Space: `O(k)` for the heap.

## Pitfalls

- Using a min-heap of all points and popping `k` times is correct but costs
  `O(n + k log n)` memory and time; a size-`k` max-heap is the usual answer.
- Taking `sqrt` introduces floating-point noise for no benefit.
- Storing the index (or the point) alongside the distance: the heap must be
  able to give back the original point.
