# Approach: binary search on the index

Take any index `i`. The papers `i, i + 1, ..., n - 1` are the `n - i` most
cited, and the least cited of them is `citations[i]`. So "at least `n - i`
papers have at least `n - i` citations" holds exactly when
`citations[i] >= n - i`.

As `i` increases, `citations[i]` does not decrease and `n - i` decreases, so
the condition is monotone: false on a prefix, true on the rest. The first
index `i` where it holds gives the largest possible `h = n - i`. If it never
holds, the search ends at `i = n` and the answer is `0`.

```python
def h_index_sorted(citations):
    n = len(citations)
    lo, hi = 0, n
    while lo < hi:
        mid = (lo + hi) // 2
        if citations[mid] >= n - mid:
            hi = mid
        else:
            lo = mid + 1
    return n - lo
```

Why is `n - i` the best `h`, and not something between? For a larger
`h = n - i + 1` we would need `citations[i - 1] >= n - i + 1`, which is the
condition at index `i - 1`, which is false because `i` is the first index
where it holds.

## Complexity

- Time: O(log n).
- Space: O(1).

## Pitfalls

- Returning `citations[i]` instead of `n - i`. The h-index counts papers; in
  `[0, 2, 3, 5, 8]` the first qualifying index has `citations[i] = 3` but in
  `[10, 10, 10]` it has `10` while the answer is `3`.
- Searching `hi` only up to `n - 1`, which misses the all-zero case where the
  answer is `0`.
- A linear scan is correct but O(n); the problem asks for O(log n).
