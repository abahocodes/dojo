def min_swaps_circular(nums: list[int]) -> int:
    n = len(nums)
    ones = sum(nums)
    if ones == 0:
        return 0
    window = sum(nums[:ones])
    best = window
    for i in range(ones, ones + n - 1):
        window += nums[i % n] - nums[i - ones]
        best = max(best, window)
    return ones - best
