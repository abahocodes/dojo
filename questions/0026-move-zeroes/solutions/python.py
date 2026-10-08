def move_zeroes(nums: list[int]) -> list[int]:
    w = 0
    for read in range(len(nums)):
        if nums[read] != 0:
            nums[w], nums[read] = nums[read], nums[w]
            w += 1
    return nums
