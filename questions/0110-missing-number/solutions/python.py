def missing_number(nums: list[int]) -> int:
    result = len(nums)
    for i, x in enumerate(nums):
        result ^= i ^ x
    return result
