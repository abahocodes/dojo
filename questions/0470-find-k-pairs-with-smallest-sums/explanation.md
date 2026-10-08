# Approach: k-way merge with a min-heap

Think of the pairs as a grid: row `i`, column `j`. Since `nums2` is sorted,
each row read left to right is already in the required order (equal sums keep
`j` increasing). The answer is the first `k` items of a merge of the rows.

A merge only needs the current front of each row. Start with column 0 of every
row that could matter. A pair in row `i` can only be among the first `k` if
every earlier row contributes a pair before it, so rows `i >= k` are never
needed. Each time you pop `(sum, i, j)`, the next candidate from that row is
`(i, j + 1)`.

Heap entries compared as tuples `(sum, i, j)` break ties exactly as the
statement asks: by sum, then `i`, then `j`.

```python
import heapq


def k_smallest_pairs(nums1, nums2, k):
    heap = [(nums1[i] + nums2[0], i, 0) for i in range(min(k, len(nums1)))]
    heapq.heapify(heap)
    result = []
    while heap and len(result) < k:
        _, i, j = heapq.heappop(heap)
        result.append([nums1[i], nums2[j]])
        if j + 1 < len(nums2):
            heapq.heappush(heap, (nums1[i] + nums2[j + 1], i, j + 1))
    return result
```

Why is this order right? Every pair not yet popped is either in the heap or
lies to the right of some heap entry in the same row; such a pair has a sum at
least as large and a larger `j`, so it can never beat the heap's minimum.

## Complexity

- Time: `O(k log min(k, n1))`: the heap never holds more than `min(k, n1)`
  entries, and there are at most `k` pops and `k` pushes.
- Space: `O(min(k, n1))` for the heap, plus the output.

## Pitfalls

- Overflow: `nums1[i] + nums2[j]` can be `±2 · 10^9`, beyond 32-bit `int`.
- Pushing both `(i + 1, j)` and `(i, j + 1)` from each pop also works but
  needs a visited set to avoid duplicates.
- Comparing only by sum gives a valid set of small sums but not the required
  order among ties.
- When `k` exceeds the number of pairs, stop when the heap empties.
