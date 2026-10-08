def sub_array_ranges(nums):
    n = len(nums)

    def total(better):
        result = 0
        stack = []
        for i in range(n + 1):
            while stack and (i == n or not better(nums[stack[-1]], nums[i])):
                j = stack.pop()
                left = stack[-1] if stack else -1
                result += nums[j] * (j - left) * (i - j)
            stack.append(i)
        return result

    return total(lambda a, b: a > b) - total(lambda a, b: a < b)
