def maximum_gap(nums: list[int]) -> int:
    n = len(nums)
    if n < 2:
        return 0
    lo, hi = min(nums), max(nums)
    if lo == hi:
        return 0
    size = max(1, (hi - lo) // (n - 1))
    count = (hi - lo) // size + 1
    bucket_min = [None] * count
    bucket_max = [None] * count
    for v in nums:
        b = (v - lo) // size
        if bucket_min[b] is None or v < bucket_min[b]:
            bucket_min[b] = v
        if bucket_max[b] is None or v > bucket_max[b]:
            bucket_max[b] = v
    best = 0
    prev = lo
    for b in range(count):
        if bucket_min[b] is not None:
            best = max(best, bucket_min[b] - prev)
            prev = bucket_max[b]
    return best
