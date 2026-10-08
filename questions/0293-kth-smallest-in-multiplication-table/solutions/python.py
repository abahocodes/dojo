def find_kth_number(m: int, n: int, k: int) -> int:
    if m > n:
        m, n = n, m

    def count_at_most(x: int) -> int:
        full_rows = min(m, x // n)
        total = full_rows * n
        for i in range(full_rows + 1, min(m, x) + 1):
            total += x // i
        return total

    lo, hi = 1, m * n
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
