def smallest_divisor(nums: list[int], threshold: int) -> int:
    def total(d: int) -> int:
        return sum((x + d - 1) // d for x in nums)

    lo, hi = 1, max(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if total(mid) <= threshold:
            hi = mid
        else:
            lo = mid + 1
    return lo
