# Approach: two heaps with lazy deletion

Keep the window split into two halves:

- `low`, a max-heap holding the smaller half, and
- `high`, a min-heap holding the larger half,

so that every live value in `low` is `<=` every live value in `high`, and
`low` holds either the same number of live values as `high` or one more. Then
the median is the top of `low` when `k` is odd, and the average of both tops
when `k` is even.

Adding a value: push it into `low` if it is at most `low`'s top, otherwise into
`high`, then move one top across if the sizes drift out of balance.

Removing the value that leaves the window is the tricky part, because a heap
can only pop its top. Instead of searching for it, record it in a `delayed`
counter and decrement the live size of the half it belongs to (it belongs to
`low` exactly when it is `<=` `low`'s top). The stale copy stays buried until
it reaches the top of its heap, at which point `prune` discards it. Pruning
after every operation that might expose a stale top keeps both tops valid, so
medians and comparisons always use live values.

```python
import heapq
from collections import defaultdict

def median_sliding_window(nums, k):
    low, high = [], []          # low stores negated values
    delayed = defaultdict(int)
    low_size = high_size = 0

    def prune(heap, sign):
        while heap and delayed[sign * heap[0]]:
            delayed[sign * heap[0]] -= 1
            heapq.heappop(heap)

    def rebalance():
        nonlocal low_size, high_size
        if low_size > high_size + 1:
            heapq.heappush(high, -heapq.heappop(low))
            low_size -= 1; high_size += 1
            prune(low, -1)
        elif low_size < high_size:
            heapq.heappush(low, -heapq.heappop(high))
            low_size += 1; high_size -= 1
            prune(high, 1)

    result = []
    for i, x in enumerate(nums):
        if not low or x <= -low[0]:
            heapq.heappush(low, -x); low_size += 1
        else:
            heapq.heappush(high, x); high_size += 1
        rebalance()
        if i >= k:
            out = nums[i - k]
            delayed[out] += 1
            if out <= -low[0]:
                low_size -= 1
                if out == -low[0]:
                    prune(low, -1)
            else:
                high_size -= 1
                if high and out == high[0]:
                    prune(high, 1)
            rebalance()
        if i >= k - 1:
            result.append(float(-low[0]) if k % 2 else (-low[0] + high[0]) / 2)
    return result
```

## Complexity

- Time: O(n log n). Each value is pushed and popped a constant number of
  times; stale entries can make a heap grow up to O(n), so each operation is
  O(log n).
- Space: O(n) for the heaps and the `delayed` counter.

## Pitfalls

- Overflow when averaging: `low + high` of two values near `2^31 - 1` does not
  fit a 32-bit int. Convert to a 64-bit integer or a double before adding.
- Rebalancing by the heaps' physical lengths. Stale entries inflate them; only
  the live sizes tell you which half is too big.
- Forgetting to prune after moving a top across. The move can expose a stale
  value on the heap you popped from.
- Integer division for even `k`: `(3 + 4) / 2` must be `3.5`, not `3`.
