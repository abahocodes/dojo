def split_array(nums: list[int], k: int) -> int:
    def pieces_needed(cap: int) -> int:
        pieces, current = 1, 0
        for x in nums:
            if current + x > cap:
                pieces += 1
                current = x
            else:
                current += x
        return pieces

    lo, hi = max(nums), sum(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if pieces_needed(mid) <= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
