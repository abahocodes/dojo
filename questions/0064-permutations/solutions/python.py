def permute(nums: list[int]) -> list[list[int]]:
    result, current = [], []
    used = [False] * len(nums)

    def backtrack():
        if len(current) == len(nums):
            result.append(current[:])
            return
        for i, x in enumerate(nums):
            if not used[i]:
                used[i] = True
                current.append(x)
                backtrack()
                current.pop()
                used[i] = False

    backtrack()
    return result
