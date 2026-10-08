# Approach: sort, then a window of size k

Suppose an optimal pick has lowest score `lo` and highest `hi`. Every score
between `lo` and `hi` could be picked instead of another one without changing
the spread, so after sorting there is an optimal pick that is a contiguous run
of `k` sorted scores. The spread of the run starting at `i` is
`sorted[i + k - 1] - sorted[i]`; take the minimum over all runs.

```python
def minimum_difference(nums, k):
    s = sorted(nums)
    return min(s[i + k - 1] - s[i] for i in range(len(s) - k + 1))
```

## Complexity

- Time: O(n log n) for the sort; the scan is O(n).
- Space: O(n) for the sorted copy (O(1) extra if you sort in place).

## Pitfalls

- Looking only at windows of the original, unsorted array. The picked scores
  do not have to be adjacent in `nums`.
- Off-by-one in the window: a run of `k` scores starting at `i` ends at
  `i + k - 1`, and the last start is `n - k`.
- Forgetting `k == 1`, where every window has spread `0`.
