from math import gcd

LIMIT = 2 * 10**9


def nth_ugly_number(n: int, a: int, b: int, c: int) -> int:
    def lcm(p: int, q: int) -> int:
        return min(p // gcd(p, q) * q, LIMIT + 1)

    ab, ac, bc = lcm(a, b), lcm(a, c), lcm(b, c)
    abc = lcm(ab, c)

    def count(x: int) -> int:
        return x // a + x // b + x // c - x // ab - x // ac - x // bc + x // abc

    lo, hi = 1, LIMIT
    while lo < hi:
        mid = (lo + hi) // 2
        if count(mid) >= n:
            hi = mid
        else:
            lo = mid + 1
    return lo
