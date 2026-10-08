from math import gcd


def nth_magical_number(n: int, a: int, b: int) -> int:
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
