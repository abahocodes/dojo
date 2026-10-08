def maximum_subarray_sum_distinct(nums: list[int], k: int) -> int:
    count = {}
    dup = 0
    window = 0
    best = 0
    for i, value in enumerate(nums):
        window += value
        count[value] = count.get(value, 0) + 1
        if count[value] == 2:
            dup += 1
        if i >= k:
            old = nums[i - k]
            window -= old
            count[old] -= 1
            if count[old] == 1:
                dup -= 1
        if i >= k - 1 and dup == 0:
            best = max(best, window)
    return best
