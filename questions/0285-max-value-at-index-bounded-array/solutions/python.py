def max_value_at_index(n: int, index: int, max_sum: int) -> int:
    def side(v: int, length: int) -> int:
        if length >= v - 1:
            return (v - 1) * v // 2 + (length - v + 1)
        return length * v - length * (length + 1) // 2

    lo, hi = 1, max_sum
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if mid + side(mid, index) + side(mid, n - 1 - index) <= max_sum:
            lo = mid
        else:
            hi = mid - 1
    return lo
