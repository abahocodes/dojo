def array_pair_sum(nums: list[int]) -> int:
    return sum(sorted(nums)[::2])
