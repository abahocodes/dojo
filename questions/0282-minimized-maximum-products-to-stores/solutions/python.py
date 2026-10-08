def minimized_maximum(n: int, quantities: list[int]) -> int:
    def stores_needed(cap: int) -> int:
        return sum((q + cap - 1) // cap for q in quantities)

    lo, hi = 1, max(quantities)
    while lo < hi:
        mid = (lo + hi) // 2
        if stores_needed(mid) <= n:
            hi = mid
        else:
            lo = mid + 1
    return lo
