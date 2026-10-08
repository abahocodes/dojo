# Hints

## Hint 1
Only the positions of the ones matter. In an optimal answer the `k` ones that
end up together are `k` ones that were already **consecutive among the
ones** (no other one jumps over them), and they keep their order.

## Hint 2
Say those ones sit at `p[0] < ... < p[k-1]` and end at `t, t+1, ..., t+k-1`.
The cost is `sum |p[j] - (t + j)|`. Substitute `q[j] = p[j] - j`: the cost is
`sum |q[j] - t|`, which is minimized by taking `t` as the median of the
`q[j]`.

## Hint 3
Slide a window of `k` over the list `q` (it is non-decreasing). With prefix
sums of `q`, the cost around the median `q[mid]` is
`q[mid] * (mid - lo) - sum(q[lo..mid-1]) + sum(q[mid+1..hi]) - q[mid] * (hi - mid)`,
which is O(1) per window.
