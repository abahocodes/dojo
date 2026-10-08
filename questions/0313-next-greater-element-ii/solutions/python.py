def next_greater_circular(nums: list[int]) -> list[int]:
    n = len(nums)
    result = [-1] * n
    stack = []
    for j in range(2 * n):
        x = nums[j % n]
        while stack and nums[stack[-1]] < x:
            result[stack.pop()] = x
        if j < n:
            stack.append(j)
    return result
