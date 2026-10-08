def max_width_ramp(nums):
    stack = []
    for i, x in enumerate(nums):
        if not stack or x < nums[stack[-1]]:
            stack.append(i)
    best = 0
    for j in range(len(nums) - 1, -1, -1):
        while stack and nums[stack[-1]] <= nums[j]:
            best = max(best, j - stack.pop())
    return best
