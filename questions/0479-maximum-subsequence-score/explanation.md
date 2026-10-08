# Approach: sort by the multiplier, keep the best k sums in a heap

Process indices in decreasing order of `nums2`. When we reach index `i`, every
index seen so far has `nums2 >= nums2[i]`, so if we pick `k` of them that
include `i`, the minimum is `nums2[i]`. To maximize the sum we want the `k`
largest `nums1` values among the processed indices.

A min-heap of size `k` tracks those values: push `nums1[i]`, and if the heap
grows past `k`, pop the smallest. It may pop `nums1[i]` itself; then the
candidate `sum * nums2[i]` uses a minimum that is too small, which only
underestimates a pick already counted at an earlier, larger `nums2`, so the
maximum is unaffected.

```python
import heapq


def max_score(nums1, nums2, k):
    pairs = sorted(zip(nums2, nums1), reverse=True)
    chosen = []
    total = 0
    best = 0
    for m, x in pairs:
        heapq.heappush(chosen, x)
        total += x
        if len(chosen) > k:
            total -= heapq.heappop(chosen)
        if len(chosen) == k:
            best = max(best, total * m)
    return best
```

## Complexity

- Time: O(n log n) for the sort, plus O(n log k) for the heap.
- Space: O(n) for the sorted order, O(k) for the heap.

## Pitfalls

- The product reaches `10^10 * 10^5 = 10^15`; use 64-bit integers.
- Trying all subsets or all `(i, j)` windows is far too slow.
- Only score once the heap holds exactly `k` values.
- Values may be zero, so the answer can be 0.
