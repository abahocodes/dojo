def max_min_power(stations: list[int], r: int, k: int) -> int:
    n = len(stations)
    # power[i] = sum of stations in [i - r, i + r], via a sliding window.
    power = [0] * n
    window = sum(stations[: min(n, r + 1)])
    for i in range(n):
        power[i] = window
        if i + r + 1 < n:
            window += stations[i + r + 1]
        if i - r >= 0:
            window -= stations[i - r]

    def feasible(target: int) -> bool:
        # Greedily build as far right as possible when a city falls short.
        added = [0] * (n + 1)  # difference array of extra power
        extra = 0
        used = 0
        for i in range(n):
            extra += added[i]
            have = power[i] + extra
            if have < target:
                need = target - have
                used += need
                if used > k:
                    return False
                extra += need
                end = min(n, i + 2 * r + 1)
                added[end] -= need
        return True

    lo, hi = min(power), min(power) + k
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if feasible(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
