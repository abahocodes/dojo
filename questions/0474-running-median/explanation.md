# Approach: two heaps

Maintain two heaps that together hold every number seen so far:

- `low`, a **max-heap** with the smaller half;
- `high`, a **min-heap** with the larger half.

Two invariants make the median easy to read off:

1. every value in `low` is `<=` every value in `high`;
2. `len(low) == len(high)` or `len(low) == len(high) + 1`.

With an odd count, the median is `low`'s top. With an even count, it is the
average of the two tops.

To add `x`: put it in `low` if it is at most `low`'s top (or `low` is empty),
otherwise in `high`. That keeps invariant 1. Then, if one heap got too big,
move its top to the other heap, which restores invariant 2 without breaking 1.

```python
import heapq


def running_median(nums):
    low, high = [], []  # low stores negated values (max-heap)
    medians = []
    for x in nums:
        if not low or x <= -low[0]:
            heapq.heappush(low, -x)
        else:
            heapq.heappush(high, x)
        if len(low) > len(high) + 1:
            heapq.heappush(high, -heapq.heappop(low))
        elif len(high) > len(low):
            heapq.heappush(low, -heapq.heappop(high))
        if len(low) > len(high):
            medians.append(float(-low[0]))
        else:
            medians.append((-low[0] + high[0]) / 2)
    return medians
```

## Complexity

- Time: `O(n log n)`: each arrival does a constant number of heap operations.
- Space: `O(n)` for the two heaps (plus the output).

## Pitfalls

- Integer division when averaging the two middle values: `(2 + 5) / 2` must be
  `3.5`, not `3`. Convert to floating point before dividing.
- Comparing the new value against the wrong heap, or rebalancing before
  inserting, which can leave a large value in `low`.
- Allowing `high` to become the bigger heap and then reading the median from
  `low`'s top.
