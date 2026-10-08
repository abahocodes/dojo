def find_duplicates(nums: list[int]) -> list[int]:
    result = []
    for x in nums:
        v = abs(x)
        if nums[v - 1] < 0:
            result.append(v)
        else:
            nums[v - 1] = -nums[v - 1]
    return result
