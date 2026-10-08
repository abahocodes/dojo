def find_best_value(arr: list[int], target: int) -> int:
    def capped(v: int) -> int:
        return sum(min(a, v) for a in arr)

    lo, hi = 0, max(arr)
    if capped(hi) < target:
        return hi
    while lo < hi:
        mid = (lo + hi) // 2
        if capped(mid) >= target:
            hi = mid
        else:
            lo = mid + 1
    if lo > 0 and target - capped(lo - 1) <= capped(lo) - target:
        return lo - 1
    return lo
