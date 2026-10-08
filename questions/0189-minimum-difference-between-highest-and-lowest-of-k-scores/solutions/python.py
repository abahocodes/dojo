def minimum_difference(nums: list[int], k: int) -> int:
    s = sorted(nums)
    return min(s[i + k - 1] - s[i] for i in range(len(s) - k + 1))
