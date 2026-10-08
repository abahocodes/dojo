# Approach: k-way merge with a min-heap

The next output element is always the smallest of the arrays' current front
elements. Keep those fronts in a min-heap tagged with where they came from.
Each step takes the heap's minimum, appends it, and replaces it with the next
element of the same array (or drops it if that array is exhausted).

```python
import heapq


def merge_k_sorted_arrays(arrays):
    heap = [(arr[0], a, 0) for a, arr in enumerate(arrays) if arr]
    heapq.heapify(heap)
    merged = []
    while heap:
        value, a, p = heap[0]
        merged.append(value)
        if p + 1 < len(arrays[a]):
            heapq.heapreplace(heap, (arrays[a][p + 1], a, p + 1))
        else:
            heapq.heappop(heap)
    return merged
```

The array index in each entry also breaks ties between equal values, so
Python never has to compare anything beyond integers.

**Alternative (divide and conquer):** merge the arrays in pairs, like the
upper levels of merge sort: `k` arrays become `k/2`, then `k/4`, and so on.
Every element is copied `O(log k)` times, the same bound as the heap.

## Complexity

- Time: `O(N log k)` for `N` total elements: each element passes through a
  heap of size at most `k` once.
- Space: `O(k)` for the heap, plus the `O(N)` output.

## Pitfalls

- Seeding the heap with `arrays[a][0]` for an empty array.
- Merging arrays one after another into a growing result costs `O(N · k)` in
  the worst case.
- `k = 0` must return an empty array.
