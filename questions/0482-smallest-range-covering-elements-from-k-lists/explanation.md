# Approach: k-way merge with a min-heap

Hold one element from each list: initially the first. The range they span is
`[min, max]`. The only way a later choice could be narrower is to drop the
current minimum, so advance the pointer of the list holding it.

A min-heap of `(value, list, position)` gives the minimum, and the maximum only
ever increases, so a single variable tracks it. Each step:

1. Pop `(low, r, c)`; `[low, high]` covers all lists, so it is a candidate.
   Keep it only if it is strictly narrower than the best so far: candidates
   appear in non-decreasing order of `low`, so on a tie the earlier one has the
   smaller start.
2. If list `r` has no next element, no later range can include list `r`
   without its current or an earlier element, so stop.
3. Otherwise push the next element of list `r` and update `high`.

```python
import heapq


def smallest_range(nums):
    heap = [(lst[0], r, 0) for r, lst in enumerate(nums)]
    heapq.heapify(heap)
    high = max(lst[0] for lst in nums)
    best = [heap[0][0], high]
    while True:
        low, r, c = heapq.heappop(heap)
        if high - low < best[1] - best[0]:
            best = [low, high]
        if c + 1 == len(nums[r]):
            return best
        nxt = nums[r][c + 1]
        high = max(high, nxt)
        heapq.heappush(heap, (nxt, r, c + 1))
```

## Complexity

- Time: O(N log k) where `N` is the total number of elements.
- Space: O(k) for the heap.

## Pitfalls

- Updating the best range with `<=` instead of `<` breaks the tie rule (a later
  range would have a larger or equal start).
- Stop as soon as the list that held the minimum is exhausted; continuing would
  produce ranges that miss that list.
- Duplicates within or across lists are fine; the heap handles equal values.
