# Approach: binary search on the answer with inclusion-exclusion

Let `count(x)` be the number of ugly numbers in `[1, x]`. By
inclusion-exclusion over the three divisibility conditions,

```
count(x) = x/a + x/b + x/c
         - x/lcm(a,b) - x/lcm(a,c) - x/lcm(b,c)
         + x/lcm(a,b,c)
```

with integer division. `count` is non-decreasing, and the `n`-th ugly number
is the smallest `x` with `count(x) >= n` (at that `x` the count just stepped
up, so `x` itself is ugly). Binary search `x` in `[1, 2 * 10^9]`.

The lcms can be huge (`lcm(a, b, c)` may reach 10^27). Any divisor larger than
`2 * 10^9` contributes `0` for every `x` in range, so cap the lcm: compute
`p // gcd(p, q) * q`, and if it would exceed the cap, use the cap instead.

```python
from math import gcd

LIMIT = 2 * 10**9

def nth_ugly_number(n, a, b, c):
    def lcm(p, q):
        r = p // gcd(p, q) * q
        return min(r, LIMIT + 1)

    ab, ac, bc = lcm(a, b), lcm(a, c), lcm(b, c)
    abc = lcm(ab, c)

    def count(x):
        return (x // a + x // b + x // c
                - x // ab - x // ac - x // bc + x // abc)

    lo, hi = 1, LIMIT
    while lo < hi:
        mid = (lo + hi) // 2
        if count(mid) >= n:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

Capping is safe for the triple lcm too: if `lcm(a, b)` was capped, the true
`lcm(a, b, c)` is even larger, and `lcm(LIMIT + 1, c)` is at least
`LIMIT + 1`, so it is also capped.

## Complexity

- Time: O(log(2 * 10^9)) = about 31 iterations of O(1) work, plus a few gcds.
- Space: O(1).

## Pitfalls

- Overflow: `a * b` and the lcms exceed 32 bits (and `lcm(a, b, c)` exceeds 64
  bits). Divide by the gcd before multiplying, and cap.
- Accepting any `x` with `count(x) == n`: in Example 1, `count(7)` is also 4
  but 7 is not ugly. The search must return the smallest such `x`.
- Counting numbers divisible by several of `a`, `b`, `c` more than once,
  e.g. adding `x/a + x/b + x/c` without the corrections.
