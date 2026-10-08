# Approach: always fuse the two shortest (Huffman)

Picture the fusions as a binary tree whose leaves are the original sticks.
Each original stick is paid once for every fusion above it, so the total cost
is `sum(length × depth)` over the leaves. Minimising that weighted depth is
exactly the problem Huffman coding solves: repeatedly merge the two smallest
weights.

Why the greedy is safe: in some optimal tree the two shortest sticks are
siblings at the deepest level (swapping a deeper, longer stick with a
shallower, shorter one never increases the cost). Fusing them first and
treating the result as a single stick reduces the problem to one with one
stick fewer, and the same argument applies again.

```python
import heapq


def connect_sticks(sticks):
    heap = list(sticks)
    heapq.heapify(heap)
    total = 0
    while len(heap) > 1:
        joined = heapq.heappop(heap) + heapq.heappop(heap)
        total += joined
        heapq.heappush(heap, joined)
    return total
```

**Alternative (two queues):** sort the sticks into one queue and append fused
sticks to a second queue; fused lengths come out in non-decreasing order, so
the two smallest are always at the fronts of the two queues. After sorting,
this runs in `O(n)`.

## Complexity

- Time: `O(n log n)`: `n - 1` fusions, each a few heap operations.
- Space: `O(n)` for the heap.

## Pitfalls

- Fusing sticks left to right or largest first: `[4, 1, 6, 2]` in input order
  costs 5 + 11 + 13 = 29 instead of 23.
- Sorting once and fusing neighbours in sorted order ignores that fused sticks
  can become longer than the next unfused ones.
- Overflow: the cost can reach about `10^4 · 10^4 · log2(10^4) ≈ 1.4 · 10^9`,
  right at the edge of 32-bit range, so accumulate in a 64-bit integer.
- A single stick costs `0`, not its length.
