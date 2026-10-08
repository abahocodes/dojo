# Approach: median of shifted positions, prefix sums

Let `p` be the indices of the ones. Swapping two equal values is wasted, so
in an optimal answer ones never cross each other: the ones that end up
together are some consecutive block `p[i..i+k-1]` of `p`, keeping their order,
landing on `t, t+1, ..., t+k-1`. Every adjacent swap moves one `1` by one
step, so the cost is

    sum over j of |p[i+j] - (t + j)|

Define `q[j] = p[j] - j`. Then `p[i+j] - (t+j) = q[i+j] - (t - i)`, so the cost
is `sum |q[i+j] - c|` for the free constant `c = t - i`. A sum of absolute
deviations is minimized at the median, so `c = q[mid]` with `mid = i + k//2`.
`q` is non-decreasing (ones are at distinct, increasing positions), so with
prefix sums `P` of `q`, the cost of the window `[lo, hi]` is

    q[mid] * (mid - lo) - (P[mid] - P[lo])          # left of the median
    + (P[hi + 1] - P[mid + 1]) - q[mid] * (hi - mid)  # right of the median

Take the minimum over all windows.

```python
def min_moves_k_ones(nums, k):
    q = []
    for i, x in enumerate(nums):
        if x == 1:
            q.append(i - len(q))
    prefix = [0]
    for v in q:
        prefix.append(prefix[-1] + v)
    best = None
    for lo in range(len(q) - k + 1):
        hi = lo + k - 1
        mid = lo + k // 2
        m = q[mid]
        cost = (m * (mid - lo) - (prefix[mid] - prefix[lo])
                + (prefix[hi + 1] - prefix[mid + 1]) - m * (hi - mid))
        if best is None or cost < best:
            best = cost
    return best
```

## Complexity

- Time: O(n): one pass to collect `q`, one for prefix sums, one over windows.
- Space: O(n) for `q` and the prefix sums.

## Pitfalls

- Using the median of the raw positions `p`. The targets are consecutive,
  not one point, so shift by `j` first (`q[j] = p[j] - j`); otherwise the
  formula counts moves that pile ones onto one cell.
- Overflow: prefix sums of positions reach about 5 * 10^9, so use 64-bit
  integers for them. The final answer is at most about 1.25 * 10^9 and fits
  in 32 bits.
- `k = 1` needs no moves; the formula gives 0 naturally.
