def min_sub_array_len(target: int, nums: list[int]) -> int:
    n = len(nums)
    best = n + 1
    window = 0
    left = 0
    for right in range(n):
        window += nums[right]
        while window >= target:
            if right - left + 1 < best:
                best = right - left + 1
            window -= nums[left]
            left += 1
    return 0 if best == n + 1 else best
