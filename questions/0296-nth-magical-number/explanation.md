# Approach: binary search with inclusion-exclusion

The number of magical values in `[1, x]` is

```
count(x) = x // a + x // b - x // lcm(a, b)
```

because multiples of both `a` and `b` are exactly the multiples of their least
common multiple, and they would otherwise be counted twice.

`count` is non-decreasing, so the `n`-th magical number is the smallest `x`
with `count(x) >= n`. The first `n` multiples of `min(a, b)` are all magical,
so the answer is at most `n * min(a, b)`.

```python
from math import gcd

def nth_magical_number(n, a, b):
    MOD = 10**9 + 7
    lcm = a // gcd(a, b) * b
    lo, hi = min(a, b), n * min(a, b)
    while lo < hi:
        mid = (lo + hi) // 2
        if mid // a + mid // b - mid // lcm >= n:
            hi = mid
        else:
            lo = mid + 1
    return lo % MOD
```

## Complexity

- Time: O(log(n * min(a, b)) + log(min(a, b))) for the search and the gcd.
- Space: O(1).

## Pitfalls

- The answer reaches `4 * 10^13`: search with 64-bit integers, and apply the
  modulus only to the final value. Taking it inside the search breaks the
  ordering.
- Computing `lcm` as `a * b / gcd` is fine here, but dividing first
  (`a / gcd * b`) avoids overflow in general.
- When `a == b` (or one divides the other), inclusion-exclusion still handles
  it: `lcm` equals the larger value.
- The smallest `x` with `count(x) >= n` is always magical itself; searching for
  `count(x) == n` can return a non-magical number.
