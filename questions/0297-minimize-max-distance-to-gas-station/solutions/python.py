def minmax_gas_dist(stations: list[int], k: int) -> float:
    gaps = [b - a for a, b in zip(stations, stations[1:])]

    def fits(limit: float) -> bool:
        added = 0
        for g in gaps:
            added += int(g / limit)
            if added > k:
                return False
        return True

    lo, hi = 0.0, float(max(gaps))
    for _ in range(100):
        mid = (lo + hi) / 2
        if fits(mid):
            hi = mid
        else:
            lo = mid
    return hi
