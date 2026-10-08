def maximum_candies(candies: list[int], k: int) -> int:
    lo, hi = 0, max(candies)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        shares = 0
        for c in candies:
            shares += c // mid
            if shares >= k:
                break
        if shares >= k:
            lo = mid
        else:
            hi = mid - 1
    return lo
