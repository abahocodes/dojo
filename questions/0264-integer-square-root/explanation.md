# Approach: binary search on the answer

`r * r` grows with `r`, so "`r * r <= x`" is true for `r = 0..answer` and
false afterwards. Binary search for the last `r` where it holds.

Bound the search: because `x <= 2^52`, the answer is at most `2^26`, so search
`[0, min(x, 2^26)]`. This also keeps `mid * mid <= 2^52`, which is exact even
in JavaScript doubles and far from 64-bit overflow.

```python
def my_sqrt(x):
    lo, hi = 0, min(x, 1 << 26)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if mid * mid <= x:
            lo = mid
        else:
            hi = mid - 1
    return lo
```

Invariant: `lo * lo <= x` and the answer is in `[lo, hi]`. Rounding `mid` up
guarantees `mid > lo`, so every step shrinks the window.

Newton's method (`r = (r + x / r) / 2` with integer division, starting at
`r = x`) is another classic answer and converges in a handful of steps.

## Complexity

- Time: `O(log x)`, about 27 iterations.
- Space: `O(1)`.

## Pitfalls

- Overflow: with `hi = x` and `x` near `2^52`, `mid * mid` is about `2^102`
  and overflows 64-bit integers (and loses precision in JavaScript). Bound
  `hi` or compare `mid <= x / mid` instead.
- Using `mid = (lo + hi) / 2` together with `lo = mid`: when `hi = lo + 1`
  the loop never ends.
- `x = 0` and `x = 1` are their own square roots; make sure the bounds allow
  them.
- Taking a floating-point square root and truncating it can be off by one for
  large inputs, and it is not allowed here anyway.
