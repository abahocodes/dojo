def count_subarrays_median_k(nums: list[int], k: int) -> int:
    p = nums.index(k)
    right = {0: 1}
    bal = 0
    for i in range(p + 1, len(nums)):
        bal += 1 if nums[i] > k else -1
        right[bal] = right.get(bal, 0) + 1
    total = 0
    bal = 0
    for i in range(p, -1, -1):
        if i < p:
            bal += 1 if nums[i] > k else -1
        total += right.get(-bal, 0) + right.get(1 - bal, 0)
    return total
