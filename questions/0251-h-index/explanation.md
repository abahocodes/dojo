# Approach: bucket counts, then sweep from the top

The h-index never exceeds `n`, so any citation count above `n` can be capped
at `n` without changing the answer. Count the papers into buckets
`0..n` by `min(c, n)`. Sweeping `h` from `n` down to `0` while accumulating
bucket sizes gives, at each step, the number of papers with at least `h`
citations. The first `h` (the largest) where that number is at least `h` is
the h-index.

```python
def h_index(citations):
    n = len(citations)
    buckets = [0] * (n + 1)  # buckets[n] holds every paper with >= n citations
    for c in citations:
        buckets[min(c, n)] += 1
    papers = 0
    for h in range(n, -1, -1):
        papers += buckets[h]
        if papers >= h:
            return h
    return 0
```

The sweep always stops by `h = 0`, since `papers >= 0` holds trivially.

A sorting solution is also fine: sort descending and return the number of
positions `i` with `citations[i] >= i + 1`.

## Complexity

- Time: O(n): one counting pass and one sweep.
- Space: O(n) for the buckets.

## Pitfalls

- Indexing buckets by the raw citation count: counts can exceed `n`, so cap
  them first.
- Off-by-one in the sorted version: position `i` (0-based) needs at least
  `i + 1` citations.
- Returning the number of cited papers: `[100, 100]` has h-index `2`, and
  `[0, 0, 0]` has h-index `0`.
