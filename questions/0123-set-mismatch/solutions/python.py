def find_error_nums(nums: list[int]) -> list[int]:
    seen = [False] * (len(nums) + 1)
    dup = 0
    total = 0
    for x in nums:
        if seen[x]:
            dup = x
        seen[x] = True
        total += x
    n = len(nums)
    missing = n * (n + 1) // 2 - (total - dup)
    return [dup, missing]
