def minimum_size(nums: list[int], max_operations: int) -> int:
    lo, hi = 1, max(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        ops = 0
        for b in nums:
            ops += (b - 1) // mid
            if ops > max_operations:
                break
        if ops <= max_operations:
            hi = mid
        else:
            lo = mid + 1
    return lo
