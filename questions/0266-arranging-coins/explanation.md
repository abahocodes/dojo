# Approach: binary search on the row count

`k` complete rows need `T(k) = k * (k + 1) / 2` coins, and `T` is
increasing, so the valid `k` form a prefix `0..answer`. Binary search for its
last element.

For the upper bound, `T(k) <= n <= 2^52` means `k * (k + 1) <= 2^53`, so
`k <= 94906265`. Searching `[0, min(n, 94906266)]` keeps `mid * (mid + 1)`
below `2^54`. That product is always even, and even integers below `2^54` are
represented exactly by JavaScript doubles, so the comparison is exact there as
well as in 64-bit integers.

```python
def arrange_coins(n):
    lo, hi = 0, min(n, 94906266)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if mid * (mid + 1) // 2 <= n:
            lo = mid
        else:
            hi = mid - 1
    return lo
```

There is also a closed form: the answer is
`floor((sqrt(8n + 1) - 1) / 2)`. With an exact integer square root that is
correct, but a floating-point `sqrt` can be off by one for `n` this large.

## Complexity

- Time: `O(log n)`, about 27 iterations.
- Space: `O(1)`.

## Pitfalls

- Simulating row by row: up to about `9.5 * 10^7` iterations, too slow in
  the slower languages.
- Overflow with an unbounded `hi = n`: `mid * (mid + 1)` would be near
  `2^104`.
- Floating-point formulas: `sqrt(8n + 1)` loses precision once `8n + 1`
  exceeds `2^53`.
- Returning the index of the incomplete row instead of the count of complete
  ones.
