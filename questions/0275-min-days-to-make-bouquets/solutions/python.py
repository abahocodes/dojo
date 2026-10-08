def min_days(bloom_day: list[int], m: int, k: int) -> int:
    if m * k > len(bloom_day):
        return -1

    def bouquets(day: int) -> int:
        made = run = 0
        for b in bloom_day:
            if b <= day:
                run += 1
                if run == k:
                    made += 1
                    run = 0
            else:
                run = 0
        return made

    lo, hi = min(bloom_day), max(bloom_day)
    while lo < hi:
        mid = (lo + hi) // 2
        if bouquets(mid) >= m:
            hi = mid
        else:
            lo = mid + 1
    return lo
