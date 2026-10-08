# Approach: binary search for the root

`r * r` is strictly increasing for `r >= 1`, so searching for an `r` with
`r * r == num` is an ordinary binary search over `r`. Since `num <= 2^52`, any
root is at most `2^26`, which bounds the search and keeps every `mid * mid` at
most `2^52`: no 64-bit overflow, and exact in JavaScript doubles.

```python
def is_perfect_square(num):
    lo, hi = 1, min(num, 1 << 26)
    while lo <= hi:
        mid = (lo + hi) // 2
        square = mid * mid
        if square == num:
            return True
        if square < num:
            lo = mid + 1
        else:
            hi = mid - 1
    return False
```

A cute alternative: perfect squares are sums of consecutive odd numbers
(`1 + 3 + 5 + ...`), so subtracting odd numbers until you hit zero or go
negative also works, but that takes `O(sqrt(num))` steps (about `6.7 * 10^7`
at the top of the range).

## Complexity

- Time: `O(log num)`, about 27 iterations.
- Space: `O(1)`.

## Pitfalls

- Searching up to `num` itself without bounding: `mid * mid` can overflow
  64-bit integers for `num` near `2^52`.
- Computing a floating-point square root and checking `r * r == num`. It is
  not allowed here, and rounding can mislead you for large inputs.
- `num = 1` is a perfect square (`1 * 1`).
