def maximum_score(nums, k):
    n = len(nums)
    i = j = k
    low = nums[k]
    best = low
    while i > 0 or j < n - 1:
        if i == 0 or (j < n - 1 and nums[j + 1] > nums[i - 1]):
            j += 1
            low = min(low, nums[j])
        else:
            i -= 1
            low = min(low, nums[i])
        best = max(best, low * (j - i + 1))
    return best
