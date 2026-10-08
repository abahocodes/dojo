def count_subarrays_max_k(nums: list[int], k: int) -> int:
    m = max(nums)
    count = 0
    left = 0
    total = 0
    for value in nums:
        if value == m:
            count += 1
        while count >= k:
            if nums[left] == m:
                count -= 1
            left += 1
        total += left
    return total
