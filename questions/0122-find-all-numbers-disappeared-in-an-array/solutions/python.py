def find_disappeared_numbers(nums: list[int]) -> list[int]:
    # Mark value v as seen by making nums[v - 1] negative.
    for x in nums:
        i = abs(x) - 1
        if nums[i] > 0:
            nums[i] = -nums[i]
    return [i + 1 for i, x in enumerate(nums) if x > 0]
