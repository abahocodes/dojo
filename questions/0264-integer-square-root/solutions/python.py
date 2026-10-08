def my_sqrt(x: int) -> int:
    lo, hi = 0, min(x, 1 << 26)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if mid * mid <= x:
            lo = mid
        else:
            hi = mid - 1
    return lo
