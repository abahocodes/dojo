def max_frequency(nums: list[int], k: int) -> int:
    a = sorted(nums)
    left = 0
    window = 0
    best = 0
    for right, value in enumerate(a):
        window += value
        while value * (right - left + 1) - window > k:
            window -= a[left]
            left += 1
        best = max(best, right - left + 1)
    return best
