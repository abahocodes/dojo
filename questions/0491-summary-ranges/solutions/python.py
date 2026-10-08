def summary_ranges(nums: list[int]) -> list[str]:
    result = []
    i = 0
    while i < len(nums):
        j = i
        while j + 1 < len(nums) and nums[j + 1] == nums[j] + 1:
            j += 1
        result.append(str(nums[i]) if i == j else f"{nums[i]}->{nums[j]}")
        i = j + 1
    return result
