# Approach: two heaps and two pointers

The applicants still in line are: the front heap, the untouched middle
`costs[i..j]`, and the back heap. The two heaps are exactly the people in the
current windows, and the middle has never been looked at.

Each round:

1. Refill the front heap from `i` (moving right) and the back heap from `j`
   (moving left) until each holds `candidates` people or the middle is empty.
2. Compare the two tops by `(cost, index)` and hire the smaller one.

Because every front index is smaller than every back index, comparing the
tuples applies the tie-break rule correctly. When the windows overlap
(`i > j`), all remaining applicants already sit in the heaps, each exactly
once.

```python
import heapq


def total_cost(costs, k, candidates):
    left, right = [], []
    i, j = 0, len(costs) - 1
    total = 0
    for _ in range(k):
        while len(left) < candidates and i <= j:
            heapq.heappush(left, (costs[i], i))
            i += 1
        while len(right) < candidates and i <= j:
            heapq.heappush(right, (costs[j], j))
            j -= 1
        if not right or (left and left[0] <= right[0]):
            total += heapq.heappop(left)[0]
        else:
            total += heapq.heappop(right)[0]
    return total
```

## Complexity

- Time: O((k + candidates) log candidates); each applicant is pushed at most
  once.
- Space: O(candidates).

## Pitfalls

- Do not add the same applicant to both heaps when the windows overlap; the
  `i <= j` guard prevents it.
- Equal costs must go to the smaller index, which is always in the front heap
  if one tie is on each side.
- The total can reach `10^10`, so use a 64-bit accumulator.
