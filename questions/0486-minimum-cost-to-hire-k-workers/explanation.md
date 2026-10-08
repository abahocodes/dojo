# Approach: sort by ratio, keep the k smallest qualities in a heap

Because pay is proportional to quality, a group is paid a single rate `r`
per unit of quality, and worker `i` gets `r * quality[i]`. The minimum-wage
rule says `r >= wage[i] / quality[i]` for every member, so the cheapest
valid rate is the **largest ratio** in the group, and

```
cost(group) = max ratio in group * sum of qualities in group
```

Sort the workers by ratio `wage / quality`. When we reach worker `i` in this
order and make them the member with the largest ratio, every earlier worker
may join, and to minimise the cost we take the `k - 1` earlier workers with
the smallest qualities (plus `i` itself). A max-heap of qualities does this
incrementally: push the current quality, and if the heap holds more than `k`
values, pop the largest. The heap then contains the `k` smallest qualities
among workers `0..i`.

When the popped value is the current worker's own quality, the heap's group
doesn't include worker `i`, and its true rate is at most `ratio(i)`. The
formula `sum * ratio(i)` then overestimates a group that was already
evaluated at its own captain, so it never produces a wrong minimum.

```python
import heapq
from functools import cmp_to_key

def mincost_to_hire_workers(quality, wage, k):
    workers = sorted(zip(quality, wage),
                     key=cmp_to_key(lambda a, b: a[1] * b[0] - b[1] * a[0]))
    heap = []
    total_quality = 0
    best = float("inf")
    for q, w in workers:
        heapq.heappush(heap, -q)
        total_quality += q
        if len(heap) > k:
            total_quality += heapq.heappop(heap)  # remove the largest quality
        if len(heap) == k:
            best = min(best, total_quality * w / q)
    return best
```

Sorting compares ratios by cross-multiplying (`w1 * q2` against `w2 * q1`),
which is exact with integers.

## Complexity

- Time: O(n log n) for the sort and the heap operations.
- Space: O(n) for the sorted order and the heap.

## Pitfalls

- Paying each worker their own `wage[i]`. The proportionality rule forces
  everyone onto the rate of the most demanding member.
- Picking the `k` cheapest wages or the `k` lowest ratios. A low-ratio worker
  with huge quality can make the group expensive.
- Integer division in the final `sum * wage / quality`: the answer is
  fractional, so divide in floating point.
- Evaluating the cost before the heap holds `k` workers.
