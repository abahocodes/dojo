# Approach: min-heap of the k largest values

The `k`-th largest value is the smallest member of the top `k`. Keep exactly
those top values in a **min-heap** capped at `k` elements:

- while the heap holds fewer than `k` values, every new value goes in;
- once it is full, a new value `v` only matters if `v > top`: it evicts the
  current top (which drops out of the top `k` forever) and takes its place.

After each insertion from `adds`, the heap is full (guaranteed by the input)
and its top is the answer.

```python
import heapq

def kth_largest_stream(k, nums, adds):
    heap = []

    def add(v):
        if len(heap) < k:
            heapq.heappush(heap, v)
        elif v > heap[0]:
            heapq.heapreplace(heap, v)

    for v in nums:
        add(v)
    result = []
    for v in adds:
        add(v)
        result.append(heap[0])
    return result
```

## Complexity

- Time: O((n + m) log k) for `n` initial values and `m` additions.
- Space: O(k) for the heap, plus the output.

## Pitfalls

- A max-heap of everything works but makes each query O(k log n) to dig out
  the `k`-th value.
- Use `v > top`, not `>=`: replacing an equal value is harmless but wasted
  work; skipping a strictly larger one is a bug.
- `nums` may hold fewer than `k` values (even none); only read the top once
  the heap actually holds `k` values.
- Duplicates count separately; a set would merge them and give wrong answers.
