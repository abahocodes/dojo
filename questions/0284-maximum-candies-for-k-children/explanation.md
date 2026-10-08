# Approach: binary search on the share size

For a share size `x`, pile `c` can be cut into `c // x` full shares (the
remainder is wasted), so the number of children we can serve is
`count(x) = sum(c // x for c in candies)`. This count never increases as `x`
grows, so the sizes that work form a prefix `1, 2, ..., best`. Binary search
finds `best`: the largest `x` with `count(x) >= k`. If even `x = 1` fails
(the total number of candies is below `k`), the answer is `0`.

No share can exceed the largest pile, so the search range is
`[1, max(candies)]`.

```python
def maximum_candies(candies, k):
    lo, hi = 0, max(candies)  # lo always works (0 means "nothing")
    while lo < hi:
        mid = (lo + hi + 1) // 2
        shares = 0
        for c in candies:
            shares += c // mid
            if shares >= k:
                break
        if shares >= k:
            lo = mid
        else:
            hi = mid - 1
    return lo
```

## Complexity

- Time: O(n log M), where M = max(candies) <= 10^7, so about 24 passes.
- Space: O(1).

## Pitfalls

- Overflow: `k` reaches 10^12 and the share count can reach 10^12 as well, so
  both must be 64-bit. Stopping the sum early once it reaches `k` also helps.
- Starting the search at 1 without handling the "impossible" case: if the
  total candy is below `k` the answer is `0`, not `1`.
- Rounding `mid` down when the update is `lo = mid` loops forever; use the
  upper midpoint.
