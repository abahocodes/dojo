def subsets_with_dup(nums: list[int]) -> list[list[int]]:
    nums = sorted(nums)  # equal values become neighbours
    result, current = [], []

    def backtrack(start):
        result.append(current[:])
        for i in range(start, len(nums)):
            if i > start and nums[i] == nums[i - 1]:
                continue  # this value was already tried in this position
            current.append(nums[i])
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
