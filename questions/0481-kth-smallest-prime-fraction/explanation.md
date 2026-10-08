# Approach: binary search on the value, counting with two pointers

Let `count(x)` be the number of fractions strictly smaller than `x`. It grows
with `x`, and the `k`-th smallest fraction `f` is the largest fraction below
any `x` with `count(x) == k`.

Counting is linear: for denominator `arr[j]`, numerators `arr[i] < x * arr[j]`
form a prefix `arr[0..i)`, and that prefix never shrinks as `j` grows. While
counting, also record the largest fraction below `x` (compare `a/b` and `c/d`
as `a*d` vs `c*b` to stay exact).

Binary search `x` on `(0, 1)` until the count equals `k`.

```python
def kth_smallest_prime_fraction(arr, k):
    n = len(arr)
    lo, hi = 0.0, 1.0
    while True:
        mid = (lo + hi) / 2
        count = 0
        p, q = 0, 1
        i = 0
        for j in range(1, n):
            while i < j and arr[i] < mid * arr[j]:
                i += 1
            count += i
            if i > 0 and arr[i - 1] * q > p * arr[j]:
                p, q = arr[i - 1], arr[j]
        if count == k:
            return [p, q]
        if count < k:
            lo = mid
        else:
            hi = mid
```

## Complexity

- Time: O(n log(1/δ)) where δ ≈ `1 / (3*10^4)^2` is the minimum gap between
  fractions, about 60 rounds of O(n).
- Space: O(1).

An alternative uses a min-heap seeded with `arr[0] / arr[j]` for every `j`,
popping `k` times and pushing the next numerator for the popped denominator:
O(k log n), which is slower when `k` is near `n² / 2`.

## Pitfalls

- Compare fractions by cross-multiplication, not by floating-point division,
  when choosing the best candidate.
- Cross products reach `9 * 10^8`; they fit in 32 bits, but 64-bit is safer.
- The pointer `i` must stay below `j` so a fraction is never `arr[j] / arr[j]`.
