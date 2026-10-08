def count_subarrays_fixed_bounds(nums: list[int], min_k: int, max_k: int) -> int:
    total = 0
    bad = last_min = last_max = -1
    for i, v in enumerate(nums):
        if v < min_k or v > max_k:
            bad = i
        if v == min_k:
            last_min = i
        if v == max_k:
            last_max = i
        total += max(0, min(last_min, last_max) - bad)
    return total
