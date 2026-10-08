# Approach: binary search on the value range

Let `count(v)` be the number of entries `<= v`. It is non-decreasing in `v`,
and the `k`-th smallest value is exactly the smallest `v` with
`count(v) >= k`. That smallest `v` must be a matrix entry: if it were not,
`count(v - 1)` would equal `count(v)`, contradicting minimality.

`count(v)` uses the sorted rows and columns. Start at the bottom-left corner.
If `matrix[row][col] <= v`, every entry above it in that column is also
`<= v`, so add `row + 1` and move right. Otherwise this entry and everything
to its right in that row are too big, so move up. Each step leaves a row or a
column behind, so the walk takes at most `2n` steps.

```python
def kth_smallest_matrix(matrix, k):
    n = len(matrix)

    def count_at_most(v):
        count, row, col = 0, n - 1, 0
        while row >= 0 and col < n:
            if matrix[row][col] <= v:
                count += row + 1
                col += 1
            else:
                row -= 1
        return count

    lo, hi = matrix[0][0], matrix[n - 1][n - 1]
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

A min-heap that pops `k` times from the row heads also works, in
O(k log n), but it degrades toward O(n^2 log n) for large `k`.

## Complexity

- Time: O(n log R), where `R = max - min <= 2 * 10^9` (about 31 iterations).
- Space: O(1).

## Pitfalls

- Overflow: `lo + hi` can reach `±2 * 10^9`, outside 32-bit range. Use a
  64-bit midpoint (or `lo + (hi - lo) / 2` in 64 bits).
- Negative values: in languages where `/` truncates toward zero,
  `(lo + hi) / 2` rounds the wrong way for negative sums and the loop can spin
  forever. Use `lo + (hi - lo) / 2` or floor division.
- Stopping at the first `mid` with `count(mid) == k`: that `mid` may not be in
  the matrix. Keep narrowing to the smallest such value.
