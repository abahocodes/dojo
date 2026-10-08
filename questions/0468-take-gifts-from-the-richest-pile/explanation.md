# Approach: max-heap simulation

Each second touches only the largest pile, so keep all piles in a max-heap.
One round is "pop the maximum, push back its integer square root", which costs
`O(log n)`. After `k` rounds, the heap holds the final pile sizes; add them up.

Python's `heapq` is a min-heap, so store negated sizes. `heapreplace` pops and
pushes in a single sift.

```python
import heapq
import math


def pick_gifts(gifts, k):
    heap = [-g for g in gifts]
    heapq.heapify(heap)
    for _ in range(k):
        largest = -heap[0]
        heapq.heapreplace(heap, -math.isqrt(largest))
    return -sum(heap)
```

Once every pile is down to 1, further rounds change nothing, so you could stop
early when the maximum is 1. It is not needed for correctness.

## Complexity

- Time: `O(n + k log n)`: heapify in `O(n)`, then `k` heap operations.
- Space: `O(n)` for the heap.

## Pitfalls

- Summing in a 32-bit `int`: the total can reach `10^13`.
- Floating-point square roots: `sqrt` of a double is exact enough for values up
  to `10^9`, but `math.isqrt` (or an integer check) is the safe habit.
- Forgetting to negate values when using a min-heap library as a max-heap.
- Scanning the whole array every second is `O(n · k)`, about `10^8` steps at
  the limits.
