# Approach: difference array over the target sum

Every pair sum must end up equal to one target `T`, and since each element
lies in `[1, limit]`, `T` is somewhere in `[2, 2 * limit]`. For a pair
`(a, b)` with `lo = min(a, b)` and `hi = max(a, b)`, the cost of reaching `T`
is:

- `0` if `T == a + b`;
- `1` if `lo + 1 <= T <= hi + limit` (rewrite one element: replacing `hi`
  with something in `[1, limit]` gives sums `lo + 1 .. lo + limit`, replacing
  `lo` gives `hi + 1 .. hi + limit`; together they cover `lo + 1 .. hi + limit`);
- `2` otherwise (rewrite both).

So each pair contributes a piecewise-constant function of `T`. Summing `n / 2`
of them directly for every `T` costs `O(n * limit)`. Instead write each pair's
breakpoints into a difference array: start at 2 for every `T`, drop by 1 at
`lo + 1`, drop by 1 more at `a + b`, rise by 1 at `a + b + 1`, and rise by 1
at `hi + limit + 1`. A prefix sum over `T` then gives the total moves for
every target, and the answer is the minimum.

```python
def min_moves_complementary(nums, limit):
    n = len(nums)
    delta = [0] * (2 * limit + 2)
    for i in range(n // 2):
        a, b = nums[i], nums[n - 1 - i]
        lo, hi = min(a, b), max(a, b)
        delta[2] += 2
        delta[lo + 1] -= 1
        delta[hi + limit + 1] += 1
        delta[a + b] -= 1
        delta[a + b + 1] += 1
    best, moves = n, 0
    for t in range(2, 2 * limit + 1):
        moves += delta[t]
        best = min(best, moves)
    return best
```

## Complexity

- Time: O(n + limit): one pass over the pairs, one pass over the sums.
- Space: O(limit) for the difference array.

## Pitfalls

- Restricting `T` to the existing pair sums. The best target may be a sum no
  pair currently has (every pair then costs 1).
- The one-move range is `lo + 1 .. hi + limit`, not `lo + 1 .. lo + limit` or
  `2 .. 2 * limit`: you keep one element and the other can only be `1..limit`.
- `a + b` always lies inside the one-move range, so the 0-cost dip is nested
  inside the 1-cost interval. Apply both updates; do not special-case it.
- Sizing the array `2 * limit + 1` and then writing to index
  `hi + limit + 1 = 2 * limit + 1`: allocate `2 * limit + 2`.
