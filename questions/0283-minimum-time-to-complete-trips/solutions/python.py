def minimum_time(time: list[int], total_trips: int) -> int:
    def enough(t: int) -> bool:
        done = 0
        for x in time:
            done += t // x
            if done >= total_trips:
                return True
        return False

    lo, hi = 1, min(time) * total_trips
    while lo < hi:
        mid = (lo + hi) // 2
        if enough(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo
