def number_of_nice_subarrays(nums: list[int], k: int) -> int:
    # seen[p] = how many prefixes contain exactly p odd numbers.
    seen = [0] * (len(nums) + 1)
    seen[0] = 1
    odds = 0
    total = 0
    for x in nums:
        odds += x & 1
        if odds >= k:
            total += seen[odds - k]
        seen[odds] += 1
    return total
