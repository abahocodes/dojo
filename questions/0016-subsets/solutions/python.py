def subsets(nums: list[int]) -> list[list[int]]:
    result = []
    current = []

    def backtrack(start: int) -> None:
        result.append(current[:])
        for i in range(start, len(nums)):
            current.append(nums[i])
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
