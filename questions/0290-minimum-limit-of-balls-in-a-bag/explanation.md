# Approach: binary search on the penalty

Fix a candidate penalty `x`. A bag with `b` balls must be broken into at least
`ceil(b / x)` pieces of size at most `x`, and every operation increases the
number of pieces by exactly one, so the bag needs `ceil(b / x) - 1 =
(b - 1) // x` operations, and that many suffice. The total
`ops(x) = sum((b - 1) // x)` is non-increasing in `x`, so the achievable
penalties form a suffix `[answer, ...]`. Binary search the smallest `x` in
`[1, max(nums)]` with `ops(x) <= max_operations`; `x = max(nums)` always works
with zero operations.

```python
def minimum_size(nums, max_operations):
    lo, hi = 1, max(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        ops = 0
        for b in nums:
            ops += (b - 1) // mid
            if ops > max_operations:
                break
        if ops <= max_operations:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(n log M), where M = max(nums) <= 10^9 (about 30 passes).
- Space: O(1).

## Pitfalls

- Greedily splitting the largest bag in half: `[9]` with two operations
  becomes `4 + 5`, then `4 + 2 + 3`, a penalty of 4, while `3 + 3 + 3`
  achieves 3.
- Using `b // x` operations instead of `(b - 1) // x`: a bag of exactly `x`
  balls needs no split.
- Overflow: with `x = 1` the operation count can reach about 10^14. Use a
  64-bit sum or stop as soon as it passes `max_operations`.
