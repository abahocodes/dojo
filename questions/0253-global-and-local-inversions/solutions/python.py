def is_ideal_permutation(nums: list[int]) -> bool:
    best = -1  # max of nums[0..j-2]
    for j in range(2, len(nums)):
        best = max(best, nums[j - 2])
        if best > nums[j]:
            return False
    return True
