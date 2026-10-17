# Approach: offline queries, sort + min-heap

Answer the queries **offline**: sort them (remembering their original
positions) and sweep from the smallest to the largest. Sort the ranges by
their left end, and keep a pointer into that list.

For the current query `q`:

1. Push every range with `left <= q` onto a min-heap of `(size, right)`.
   Those are exactly the ranges that have started by `q`.
2. While the heap's top has `right < q`, pop it. It ended before `q`, and
   because queries only grow, it can never contain a later query either, so
   discarding it for good is safe.
3. The top is now the smallest range with `left <= q <= right`: the answer.
   An empty heap means `-1`.

Ranges that are not on top but already ended are left in the heap; they are
cleaned up lazily, only when they reach the top.

```python
import heapq

def min_interval(intervals, queries):
    intervals = sorted(intervals)
    order = sorted(range(len(queries)), key=lambda i: queries[i])
    answer = [-1] * len(queries)
    heap = []
    i = 0
    for qi in order:
        q = queries[qi]
        while i < len(intervals) and intervals[i][0] <= q:
            left, right = intervals[i]
            heapq.heappush(heap, (right - left + 1, right))
            i += 1
        while heap and heap[0][1] < q:
            heapq.heappop(heap)
        if heap:
            answer[qi] = heap[0][0]
    return answer
```

## Complexity

- Time: O(n log n + m log m) for `n` ranges and `m` queries: two sorts, and
  each range is pushed and popped at most once.
- Space: O(n + m) for the heap, the sorted copies and the answer.

## Pitfalls

- Answering in sorted order but writing answers in sorted order too: keep the
  original index of each query and store the answer there.
- Popping only ranges with `right < q` is correct; popping ranges whose size
  is just "not smallest" loses candidates for later queries.
- The heap must be keyed by size, not by right end: the range that ends
  soonest is not necessarily the smallest.
- Repeated queries are fine: the second one sees the same heap state.
- Ranges are closed, so `left <= q` and `right >= q` both include equality,
  and the size is `right - left + 1`, not `right - left`.
