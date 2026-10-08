def min_operations_reduce_x(nums: list[int], x: int) -> int:
    target = sum(nums) - x
    if target < 0:
        return -1
    best = -1
    window = 0
    left = 0
    for right, value in enumerate(nums):
        window += value
        while window > target:
            window -= nums[left]
            left += 1
        if window == target:
            best = max(best, right - left + 1)
    return -1 if best == -1 else len(nums) - best
