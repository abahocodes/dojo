# Approach: binary-search counts, then rank by (count, index)

Because each row is a block of 1s followed by 0s, the number of soldiers is
the index of the first `0`, which binary search finds in O(log cols).

The strength order is "fewer soldiers first, then smaller index first", which
is exactly the natural order of the pairs `(soldiers, index)`. Sort the row
indices by that key and keep the first `k`.

```python
def k_weakest_rows(mat, k):
    def soldiers(row):
        lo, hi = 0, len(row)
        while lo < hi:
            mid = (lo + hi) // 2
            if row[mid] == 1:
                lo = mid + 1
            else:
                hi = mid
        return lo

    ranked = sorted(range(len(mat)), key=lambda i: (soldiers(mat[i]), i))
    return ranked[:k]
```

**Heap variant:** push `(soldiers, index)` into a max-heap capped at `k`
entries, evicting the strongest whenever it grows past `k`; at the end, pop
everything and reverse. That's O(rows · log k) after counting, useful when
`k` is much smaller than the number of rows.

## Complexity

- Time: O(rows · log cols + rows · log rows).
- Space: O(rows) for the counts and the order.

## Pitfalls

- Ties must go to the lower index. Sorting by count alone with an unstable
  sort can reorder equal rows.
- Rows can be all 0s or all 1s; make sure the binary search returns `0` and
  `cols` in those cases.
- Return row indices, not soldier counts.
