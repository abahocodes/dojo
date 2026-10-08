# Approach: a min-heap of the `k` largest values

Walk the list once while keeping the `k` largest values seen so far in a
min-heap. The heap's top is the smallest of those `k` values, which is exactly
the `k`-th largest so far. A new value only matters if it beats that top, and
then it replaces it.

```python
import heapq

def find_kth_largest(nums, k):
    heap = nums[:k]
    heapq.heapify(heap)                 # O(k)
    for x in nums[k:]:
        if x > heap[0]:
            heapq.heapreplace(heap, x)  # pop the minimum, push x
    return heap[0]
```

**Alternatives:**

- Sort and index: `sorted(nums)[-k]`. O(n log n), perfectly fine in practice.
- Quickselect: partition around a pivot like quicksort, but only recurse into
  the side that contains position `n - k`. O(n) on average (with a random
  pivot), O(n^2) in the worst case.
- Counting sort: values lie in `[-10^4, 10^4]`, so count them and walk the
  counts from the top. O(n + range).

## Complexity

- Time: O(n log k): each of the `n` values costs at most one heap operation on
  a heap of size `k`.
- Space: O(k) for the heap.

## Pitfalls

- Use a **min**-heap of size `k`, not a max-heap. The element you evict is the
  smallest of the candidates.
- Duplicates are not collapsed: the answer for `[9, 9, 7]` with `k = 2` is `9`,
  not `7`. Don't deduplicate with a set.
- `k` counts from 1, so the answer of a sorted ascending list is at index
  `n - k`, not `k`.
