# Approach: binary search on the peak value

Fix `nums[index] = v`. Every other element is at least
`max(v - distance, 1)` (it can drop by at most 1 per step and must stay
positive), and the array that uses exactly those values is valid. So the
minimum total for peak `v` is `v + side(v, left) + side(v, right)`, where
`left = index`, `right = n - 1 - index`, and `side(v, L)` sums
`max(v - j, 1)` for `j = 1..L`:

- if `L >= v - 1`, the side descends `v - 1, ..., 1` and then has
  `L - (v - 1)` ones: `(v - 1) * v / 2 + (L - v + 1)`;
- otherwise it is `v - 1, ..., v - L`: `L * v - L * (L + 1) / 2`.

The minimum total is increasing in `v`, so binary search the largest `v` in
`[1, max_sum]` whose minimum total fits. `v = 1` always fits because
`n <= max_sum`.

```python
def max_value_at_index(n, index, max_sum):
    def side(v, length):
        if length >= v - 1:
            return (v - 1) * v // 2 + (length - v + 1)
        return length * v - length * (length + 1) // 2

    lo, hi = 1, max_sum
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if mid + side(mid, index) + side(mid, n - 1 - index) <= max_sum:
            lo = mid
        else:
            hi = mid - 1
    return lo
```

## Complexity

- Time: O(log max_sum), about 30 iterations of O(1) work.
- Space: O(1).

## Pitfalls

- Overflow: `v * v` reaches 10^18, far beyond 32 bits. Use 64-bit integers.
- Forgetting the floor of 1: once the descent reaches 1 the remaining cells
  still cost 1 each, they do not drop to 0 or below.
- Simulating the array element by element: `n` is up to 10^9.
