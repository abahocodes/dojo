def min_speed_on_time(dist: list[int], hour: float) -> int:
    total = round(hour * 100)
    last = dist[-1] * 100

    def on_time(speed: int) -> bool:
        whole = 0
        for i in range(len(dist) - 1):
            whole += (dist[i] + speed - 1) // speed
        rest = total - whole * 100
        return rest >= 0 and (rest >= last or last <= rest * speed)

    lo, hi = 1, 10**7
    if not on_time(hi):
        return -1
    while lo < hi:
        mid = (lo + hi) // 2
        if on_time(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo
