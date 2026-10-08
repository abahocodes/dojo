def max_run_time(n: int, batteries: list[int]) -> int:
    def can_run(minutes: int) -> bool:
        return sum(min(b, minutes) for b in batteries) >= n * minutes

    lo, hi = 0, sum(batteries) // n
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if can_run(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
