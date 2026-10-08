def find_unsorted_subarray(nums: list[int]) -> int:
    n = len(nums)
    end = -1
    running_max = nums[0]
    for i in range(1, n):
        if nums[i] < running_max:
            end = i
        else:
            running_max = nums[i]
    if end == -1:
        return 0
    start = n
    running_min = nums[-1]
    for i in range(n - 2, -1, -1):
        if nums[i] > running_min:
            start = i
        else:
            running_min = nums[i]
    return end - start + 1
