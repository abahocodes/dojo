def count_bad_pairs(nums: list[int]) -> int:
    seen = {}
    good = 0
    for j, x in enumerate(nums):
        key = x - j
        good += seen.get(key, 0)
        seen[key] = seen.get(key, 0) + 1
    n = len(nums)
    return n * (n - 1) // 2 - good
