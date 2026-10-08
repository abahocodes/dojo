def min_eating_speed(piles: list[int], h: int) -> int:
    lo, hi = 1, max(piles)
    while lo < hi:
        v = (lo + hi) // 2
        hours = 0
        for p in piles:
            hours += (p + v - 1) // v
        if hours <= h:
            hi = v
        else:
            lo = v + 1
    return lo
