def min_moves_k_ones(nums: list[int], k: int) -> int:
    q = []
    for i, x in enumerate(nums):
        if x == 1:
            q.append(i - len(q))
    prefix = [0]
    for v in q:
        prefix.append(prefix[-1] + v)
    best = None
    for lo in range(len(q) - k + 1):
        hi = lo + k - 1
        mid = lo + k // 2
        m = q[mid]
        cost = (m * (mid - lo) - (prefix[mid] - prefix[lo])
                + (prefix[hi + 1] - prefix[mid + 1]) - m * (hi - mid))
        if best is None or cost < best:
            best = cost
    return best
